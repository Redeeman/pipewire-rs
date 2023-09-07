// Copyright The pipewire-rs Contributors.
// SPDX-License-Identifier: MIT

use bitflags::bitflags;
use libc::c_void;
use std::{fmt, mem};
use std::{pin::Pin, ptr};

use crate::{
    proxy::{Listener, Proxy, ProxyT},
    spa::Direction,
    types::ObjectType,
};
use spa::{dict::ForeignDict, pod::Pod, spa_interface_call_method};

#[derive(Debug)]
pub struct Port {
    proxy: Proxy,
}

impl Port {
    // TODO: add non-local version when we'll bind pw_thread_loop_start()
    #[must_use]
    pub fn add_listener_local(&self) -> PortListenerLocalBuilder {
        PortListenerLocalBuilder {
            port: self,
            cbs: ListenerLocalCallbacks::default(),
        }
    }

    /// Subscribe to parameter changes
    ///
    /// Automatically emit `param` events for the given ids when they are changed
    // FIXME: Return result?
    pub fn subscribe_params(&self, ids: &[spa::param::ParamType]) {
        unsafe {
            spa_interface_call_method!(
                self.proxy.as_ptr(),
                pw_sys::pw_port_methods,
                subscribe_params,
                ids.as_ptr() as *mut _,
                ids.len().try_into().unwrap()
            );
        }
    }

    /// Enumerate node parameters
    ///
    /// Start enumeration of node parameters. For each param, a
    /// param event will be emitted.
    ///
    /// # Parameters
    /// `seq`: a sequence number to place in the reply \
    /// `id`: the parameter id to enum, or [`None`] to allow any id \
    /// `start`: the start index or 0 for the first param \
    /// `num`: the maximum number of params to retrieve ([`u32::MAX`] may be used to retrieve all params)
    // FIXME: Add filter parameter
    // FIXME: Return result?
    pub fn enum_params(&self, seq: i32, id: Option<spa::param::ParamType>, start: u32, num: u32) {
        let id = id.map(|id| id.as_raw()).unwrap_or(crate::constants::ID_ANY);

        unsafe {
            spa_interface_call_method!(
                self.proxy.as_ptr(),
                pw_sys::pw_node_methods,
                enum_params,
                seq,
                id,
                start,
                num,
                std::ptr::null()
            );
        }
    }
}

impl ProxyT for Port {
    fn type_() -> ObjectType {
        ObjectType::Port
    }

    fn upcast(self) -> Proxy {
        self.proxy
    }

    fn upcast_ref(&self) -> &Proxy {
        &self.proxy
    }

    unsafe fn from_proxy_unchecked(proxy: Proxy) -> Self
    where
        Self: Sized,
    {
        Self { proxy }
    }
}

#[derive(Default)]
struct ListenerLocalCallbacks {
    #[allow(clippy::type_complexity)]
    info: Option<Box<dyn Fn(&PortInfo)>>,
    #[allow(clippy::type_complexity)]
    param: Option<Box<dyn Fn(i32, spa::param::ParamType, u32, u32, Option<&Pod>)>>,
}

pub struct PortListenerLocalBuilder<'a> {
    port: &'a Port,
    cbs: ListenerLocalCallbacks,
}

pub struct PortInfo {
    ptr: ptr::NonNull<pw_sys::pw_port_info>,
    props: Option<ForeignDict>,
}

impl PortInfo {
    fn new(ptr: ptr::NonNull<pw_sys::pw_port_info>) -> Self {
        let props_ptr = unsafe { ptr.as_ref().props };
        let props = ptr::NonNull::new(props_ptr).map(|ptr| unsafe { ForeignDict::from_ptr(ptr) });

        Self { ptr, props }
    }

    pub fn id(&self) -> u32 {
        unsafe { self.ptr.as_ref().id }
    }

    pub fn direction(&self) -> Direction {
        let direction = unsafe { self.ptr.as_ref().direction };

        Direction::from_raw(direction)
    }

    pub fn change_mask(&self) -> PortChangeMask {
        let mask = unsafe { self.ptr.as_ref().change_mask };
        PortChangeMask::from_bits_retain(mask)
    }

    pub fn props(&self) -> Option<&ForeignDict> {
        self.props.as_ref()
    }

    /// Get the param infos for the port.
    pub fn params(&self) -> &[spa::param::ParamInfo] {
        unsafe {
            let params = self.ptr.as_ref().params;

            if params.is_null() {
                &[]
            } else {
                std::slice::from_raw_parts(
                    params as *const _,
                    self.ptr.as_ref().n_params.try_into().unwrap(),
                )
            }
        }
    }
}

bitflags! {
    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub struct PortChangeMask: u64 {
        const PROPS = pw_sys::PW_PORT_CHANGE_MASK_PROPS as u64;
        const PARAMS = pw_sys::PW_PORT_CHANGE_MASK_PARAMS as u64;
    }
}

impl fmt::Debug for PortInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PortInfo")
            .field("id", &self.id())
            .field("direction", &self.direction())
            .field("change-mask", &self.change_mask())
            .field("props", &self.props())
            .field("params", &self.params())
            .finish()
    }
}

pub struct PortListener {
    // Need to stay allocated while the listener is registered
    #[allow(dead_code)]
    events: Pin<Box<pw_sys::pw_port_events>>,
    listener: Pin<Box<spa_sys::spa_hook>>,
    #[allow(dead_code)]
    data: Box<ListenerLocalCallbacks>,
}

impl Listener for PortListener {}

impl Drop for PortListener {
    fn drop(&mut self) {
        spa::hook::remove(*self.listener);
    }
}

impl<'a> PortListenerLocalBuilder<'a> {
    #[must_use]
    pub fn info<F>(mut self, info: F) -> Self
    where
        F: Fn(&PortInfo) + 'static,
    {
        self.cbs.info = Some(Box::new(info));
        self
    }

    #[must_use]
    pub fn param<F>(mut self, param: F) -> Self
    where
        F: Fn(i32, spa::param::ParamType, u32, u32, Option<&Pod>) + 'static,
    {
        self.cbs.param = Some(Box::new(param));
        self
    }

    #[must_use]
    pub fn register(self) -> PortListener {
        unsafe extern "C" fn port_events_info(
            data: *mut c_void,
            info: *const pw_sys::pw_port_info,
        ) {
            let callbacks = (data as *mut ListenerLocalCallbacks).as_ref().unwrap();
            let info = ptr::NonNull::new(info as *mut _).expect("info is NULL");
            let info = PortInfo::new(info);
            callbacks.info.as_ref().unwrap()(&info);
        }

        unsafe extern "C" fn port_events_param(
            data: *mut c_void,
            seq: i32,
            id: u32,
            index: u32,
            next: u32,
            param: *const spa_sys::spa_pod,
        ) {
            let callbacks = (data as *mut ListenerLocalCallbacks).as_ref().unwrap();

            let id = spa::param::ParamType::from_raw(id);
            let param = if !param.is_null() {
                unsafe { Some(Pod::from_raw(param)) }
            } else {
                None
            };

            callbacks.param.as_ref().unwrap()(seq, id, index, next, param);
        }

        let e = unsafe {
            let mut e: Pin<Box<pw_sys::pw_port_events>> = Box::pin(mem::zeroed());
            e.version = pw_sys::PW_VERSION_PORT_EVENTS;

            if self.cbs.info.is_some() {
                e.info = Some(port_events_info);
            }
            if self.cbs.param.is_some() {
                e.param = Some(port_events_param);
            }

            e
        };

        let (listener, data) = unsafe {
            let port = &self.port.proxy.as_ptr();

            let data = Box::into_raw(Box::new(self.cbs));
            let mut listener: Pin<Box<spa_sys::spa_hook>> = Box::pin(mem::zeroed());
            let listener_ptr: *mut spa_sys::spa_hook = listener.as_mut().get_unchecked_mut();

            spa_interface_call_method!(
                port,
                pw_sys::pw_port_methods,
                add_listener,
                listener_ptr.cast(),
                e.as_ref().get_ref(),
                data as *mut _
            );

            (listener, Box::from_raw(data))
        };

        PortListener {
            events: e,
            listener,
            data,
        }
    }
}
