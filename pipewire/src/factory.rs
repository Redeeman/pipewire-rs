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
use spa::dict::ForeignDict;
use spa::spa_interface_call_method;

#[derive(Debug)]
pub struct Factory {
    proxy: Proxy,
}

impl ProxyT for Factory {
    fn type_() -> ObjectType {
        ObjectType::Factory
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

impl Factory {
    // TODO: add non-local version when we'll bind pw_thread_loop_start()
    #[must_use]
    pub fn add_listener_local(&self) -> FactoryListenerLocalBuilder {
        FactoryListenerLocalBuilder {
            factory: self,
            cbs: ListenerLocalCallbacks::default(),
        }
    }
}

#[derive(Default)]
struct ListenerLocalCallbacks {
    #[allow(clippy::type_complexity)]
    info: Option<Box<dyn Fn(&FactoryInfo)>>,
}

pub struct FactoryListenerLocalBuilder<'a> {
    factory: &'a Factory,
    cbs: ListenerLocalCallbacks,
}

pub struct FactoryInfo {
    ptr: ptr::NonNull<pw_sys::pw_factory_info>,
    props: Option<ForeignDict>,
}

impl FactoryInfo {
    fn new(ptr: ptr::NonNull<pw_sys::pw_factory_info>) -> Self {
        let props_ptr = unsafe { ptr.as_ref().props };
        let props = ptr::NonNull::new(props_ptr).map(|ptr| unsafe { ForeignDict::from_ptr(ptr) });

        Self { ptr, props }
    }

    pub fn id(&self) -> u32 {
        unsafe { self.ptr.as_ref().id }
    }

    pub fn type_(&self) -> ObjectType {
        unsafe { ObjectType::from_str(CStr::from_ptr(self.ptr.as_ref().type_).to_str().unwrap()) }
    }

    pub fn version(&self) -> u32 {
        unsafe { self.ptr.as_ref().version }
    }

    pub fn change_mask(&self) -> FactoryChangeMask {
        let mask = unsafe { self.ptr.as_ref().change_mask };
        FactoryChangeMask::from_bits(mask).expect("invalid change_mask")
    }

    pub fn props(&self) -> Option<&ForeignDict> {
        self.props.as_ref()
    }
}

bitflags! {
    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub struct FactoryChangeMask: u64 {
        const PROPS = pw_sys::PW_FACTORY_CHANGE_MASK_PROPS as u64;
    }
}

impl fmt::Debug for FactoryInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FactoryInfo")
            .field("id", &self.id())
            .field("type", &self.type_())
            .field("version", &self.version())
            .field("change_mask", &self.change_mask())
            .field("props", &self.props())
            .finish()
    }
}

pub struct FactoryListener {
    // Need to stay allocated while the listener is registered
    #[allow(dead_code)]
    events: Pin<Box<pw_sys::pw_factory_events>>,
    listener: Pin<Box<spa_sys::spa_hook>>,
    #[allow(dead_code)]
    data: Box<ListenerLocalCallbacks>,
}

impl Listener for FactoryListener {}

impl Drop for FactoryListener {
    fn drop(&mut self) {
        spa::hook::remove(*self.listener);
    }
}

impl<'a> FactoryListenerLocalBuilder<'a> {
    #[must_use]
    pub fn info<F>(mut self, info: F) -> Self
    where
        F: Fn(&FactoryInfo) + 'static,
    {
        self.cbs.info = Some(Box::new(info));
        self
    }

    #[must_use]
    pub fn register(self) -> FactoryListener {
        unsafe extern "C" fn factory_events_info(
            data: *mut c_void,
            info: *const pw_sys::pw_factory_info,
        ) {
            let callbacks = (data as *mut ListenerLocalCallbacks).as_ref().unwrap();
            let info = ptr::NonNull::new(info as *mut _).expect("info is NULL");
            let info = FactoryInfo::new(info);
            callbacks.info.as_ref().unwrap()(&info);
        }

        let e = unsafe {
            let mut e: Pin<Box<pw_sys::pw_factory_events>> = Box::pin(mem::zeroed());
            e.version = pw_sys::PW_VERSION_FACTORY_EVENTS;

            if self.cbs.info.is_some() {
                e.info = Some(factory_events_info);
            }

            e
        };

        let (listener, data) = unsafe {
            let factory = &self.factory.proxy.as_ptr();

            let data = Box::into_raw(Box::new(self.cbs));
            let mut listener: Pin<Box<spa_sys::spa_hook>> = Box::pin(mem::zeroed());
            let listener_ptr: *mut spa_sys::spa_hook = listener.as_mut().get_unchecked_mut();

            spa_interface_call_method!(
                factory,
                pw_sys::pw_factory_methods,
                add_listener,
                listener_ptr.cast(),
                e.as_ref().get_ref(),
                data as *mut _
            );

            (listener, Box::from_raw(data))
        };

        FactoryListener {
            events: e,
            listener,
            data,
        }
    }
}
