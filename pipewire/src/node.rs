// Copyright The pipewire-rs Contributors.
// SPDX-License-Identifier: MIT

use bitflags::bitflags;
use libc::c_void;
use std::pin::Pin;
use std::{ffi::CStr, ptr};
use std::{fmt, mem};

use crate::{
    proxy::{Listener, Proxy, ProxyT},
    types::ObjectType,
};
use spa::{dict::ForeignDict, pod::Pod, spa_interface_call_method};

#[derive(Debug)]
pub struct Node {
    proxy: Proxy,
}

impl Node {
    // TODO: add non-local version when we'll bind pw_thread_loop_start()
    #[must_use]
    pub fn add_listener_local(&self) -> NodeListenerLocalBuilder {
        NodeListenerLocalBuilder {
            node: self,
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
                pw_sys::pw_node_methods,
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

    pub fn set_param(&self, id: spa::param::ParamType, flags: u32, param: &Pod) {
        unsafe {
            spa_interface_call_method!(
                self.proxy.as_ptr(),
                pw_sys::pw_node_methods,
                set_param,
                id.as_raw(),
                flags,
                param.as_raw_ptr()
            );
        }
    }
}

impl ProxyT for Node {
    fn type_() -> ObjectType {
        ObjectType::Node
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
    info: Option<Box<dyn Fn(&NodeInfo)>>,
    #[allow(clippy::type_complexity)]
    param: Option<Box<dyn Fn(i32, spa::param::ParamType, u32, u32, Option<&Pod>)>>,
}

pub struct NodeListenerLocalBuilder<'a> {
    node: &'a Node,
    cbs: ListenerLocalCallbacks,
}

pub struct NodeInfo {
    ptr: ptr::NonNull<pw_sys::pw_node_info>,
    props: Option<ForeignDict>,
}

impl NodeInfo {
    /// Create a `NodeInfo` from a raw `pw_sys::pw_node_info`.
    ///
    /// # Safety
    /// `ptr` must point to a valid, well aligned `pw_sys::pw_node_info`.
    fn from_raw(ptr: ptr::NonNull<pw_sys::pw_node_info>) -> Self {
        let props_ptr = unsafe { ptr.as_ref().props };
        let props = ptr::NonNull::new(props_ptr).map(|ptr| unsafe { ForeignDict::from_ptr(ptr) });

        Self { ptr, props }
    }

    pub fn id(&self) -> u32 {
        unsafe { self.ptr.as_ref().id }
    }

    pub fn max_input_ports(&self) -> u32 {
        unsafe { self.ptr.as_ref().max_input_ports }
    }

    pub fn max_output_ports(&self) -> u32 {
        unsafe { self.ptr.as_ref().max_output_ports }
    }

    pub fn change_mask(&self) -> NodeChangeMask {
        let mask = unsafe { self.ptr.as_ref().change_mask };
        NodeChangeMask::from_bits_retain(mask)
    }

    pub fn n_input_ports(&self) -> u32 {
        unsafe { self.ptr.as_ref().n_input_ports }
    }

    pub fn n_output_ports(&self) -> u32 {
        unsafe { self.ptr.as_ref().n_output_ports }
    }

    pub fn state(&self) -> NodeState {
        let state = unsafe { self.ptr.as_ref().state };
        match state {
            pw_sys::pw_node_state_PW_NODE_STATE_ERROR => {
                let error = unsafe {
                    let error = self.ptr.as_ref().error;
                    CStr::from_ptr(error).to_str().unwrap()
                };
                NodeState::Error(error)
            }
            pw_sys::pw_node_state_PW_NODE_STATE_CREATING => NodeState::Creating,
            pw_sys::pw_node_state_PW_NODE_STATE_SUSPENDED => NodeState::Suspended,
            pw_sys::pw_node_state_PW_NODE_STATE_IDLE => NodeState::Idle,
            pw_sys::pw_node_state_PW_NODE_STATE_RUNNING => NodeState::Running,
            _ => panic!("Invalid node state: {}", state),
        }
    }

    pub fn props(&self) -> Option<&ForeignDict> {
        self.props.as_ref()
    }

    /// Get the param infos for the node.
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
    pub struct NodeChangeMask: u64 {
        const INPUT_PORTS = pw_sys::PW_NODE_CHANGE_MASK_INPUT_PORTS as u64;
        const OUTPUT_PORTS = pw_sys::PW_NODE_CHANGE_MASK_OUTPUT_PORTS as u64;
        const STATE = pw_sys::PW_NODE_CHANGE_MASK_STATE as u64;
        const PROPS = pw_sys::PW_NODE_CHANGE_MASK_PROPS as u64;
        const PARAMS = pw_sys::PW_NODE_CHANGE_MASK_PARAMS as u64;
    }
}

#[derive(Debug)]
pub enum NodeState<'a> {
    Error(&'a str),
    Creating,
    Suspended,
    Idle,
    Running,
}

impl fmt::Debug for NodeInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("NodeInfo")
            .field("id", &self.id())
            .field("max-input-ports", &self.max_input_ports())
            .field("max-output-ports", &self.max_output_ports())
            .field("change-mask", &self.change_mask())
            .field("n-input-ports", &self.n_input_ports())
            .field("n-output-ports", &self.n_output_ports())
            .field("state", &self.state())
            .field("props", &self.props())
            .field("params", &self.params())
            .finish()
    }
}

pub struct NodeListener {
    // Need to stay allocated while the listener is registered
    #[allow(dead_code)]
    events: Pin<Box<pw_sys::pw_node_events>>,
    listener: Pin<Box<spa_sys::spa_hook>>,
    #[allow(dead_code)]
    data: Box<ListenerLocalCallbacks>,
}

impl Listener for NodeListener {}

impl Drop for NodeListener {
    fn drop(&mut self) {
        spa::hook::remove(*self.listener);
    }
}

impl<'a> NodeListenerLocalBuilder<'a> {
    #[must_use]
    pub fn info<F>(mut self, info: F) -> Self
    where
        F: Fn(&NodeInfo) + 'static,
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
    pub fn register(self) -> NodeListener {
        unsafe extern "C" fn node_events_info(
            data: *mut c_void,
            info: *const pw_sys::pw_node_info,
        ) {
            let callbacks = (data as *mut ListenerLocalCallbacks).as_ref().unwrap();
            let info = ptr::NonNull::new(info as *mut _).expect("info is NULL");
            let info = NodeInfo::from_raw(info);
            callbacks.info.as_ref().unwrap()(&info);
        }

        unsafe extern "C" fn node_events_param(
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
            let mut e: Pin<Box<pw_sys::pw_node_events>> = Box::pin(mem::zeroed());
            e.version = pw_sys::PW_VERSION_NODE_EVENTS;

            if self.cbs.info.is_some() {
                e.info = Some(node_events_info);
            }
            if self.cbs.param.is_some() {
                e.param = Some(node_events_param);
            }

            e
        };

        let (listener, data) = unsafe {
            let node = &self.node.proxy.as_ptr();

            let data = Box::into_raw(Box::new(self.cbs));
            let mut listener: Pin<Box<spa_sys::spa_hook>> = Box::pin(mem::zeroed());
            let listener_ptr: *mut spa_sys::spa_hook = listener.as_mut().get_unchecked_mut();

            spa_interface_call_method!(
                node,
                pw_sys::pw_node_methods,
                add_listener,
                listener_ptr.cast(),
                e.as_ref().get_ref(),
                data as *mut _
            );

            (listener, Box::from_raw(data))
        };

        NodeListener {
            events: e,
            listener,
            data,
        }
    }
}
