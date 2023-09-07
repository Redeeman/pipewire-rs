// Copyright The pipewire-rs Contributors.
// SPDX-License-Identifier: MIT

use bitflags::bitflags;
use libc::c_void;
use std::pin::Pin;
use std::{ffi::CString, ptr};
use std::{fmt, mem};

use crate::{
    permissions::Permissions,
    proxy::{Listener, Proxy, ProxyT},
    types::ObjectType,
};
use spa::dict::ForeignDict;
use spa::spa_interface_call_method;

#[derive(Debug)]
pub struct Client {
    proxy: Proxy,
}

impl ProxyT for Client {
    fn type_() -> ObjectType {
        ObjectType::Client
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

impl Client {
    // TODO: add non-local version when we'll bind pw_thread_loop_start()
    #[must_use]
    pub fn add_listener_local(&self) -> ClientListenerLocalBuilder {
        ClientListenerLocalBuilder {
            client: self,
            cbs: ListenerLocalCallbacks::default(),
        }
    }

    pub fn error(&self, id: u32, res: i32, message: &str) {
        let message = CString::new(message).expect("Null byte in message parameter");

        unsafe {
            spa_interface_call_method!(
                self.proxy.as_ptr(),
                pw_sys::pw_client_methods,
                error,
                id,
                res,
                message.as_ptr() as *const _
            );
        };
    }

    pub fn update_properties<D: crate::spa::dict::ReadableDict>(&self, properties: &D) {
        unsafe {
            spa_interface_call_method!(
                self.proxy.as_ptr(),
                pw_sys::pw_client_methods,
                update_properties,
                properties.get_dict_ptr()
            );
        }
    }

    pub fn get_permissions(&self, index: u32, num: u32) {
        unsafe {
            spa_interface_call_method!(
                self.proxy.as_ptr(),
                pw_sys::pw_client_methods,
                get_permissions,
                index,
                num
            );
        }
    }

    pub fn update_permissions(&self, permissions: &[Permissions]) {
        unsafe {
            spa_interface_call_method!(
                self.proxy.as_ptr(),
                pw_sys::pw_client_methods,
                update_permissions,
                permissions.len() as u32,
                permissions.as_ptr().cast()
            );
        }
    }
}

#[derive(Default)]
struct ListenerLocalCallbacks {
    #[allow(clippy::type_complexity)]
    info: Option<Box<dyn Fn(&ClientInfo)>>,
    permissions: Option<Box<dyn Fn(u32, &[Permissions])>>,
}

pub struct ClientListenerLocalBuilder<'a> {
    client: &'a Client,
    cbs: ListenerLocalCallbacks,
}

pub struct ClientInfo {
    ptr: ptr::NonNull<pw_sys::pw_client_info>,
    props: Option<ForeignDict>,
}

impl ClientInfo {
    fn new(ptr: ptr::NonNull<pw_sys::pw_client_info>) -> Self {
        let props_ptr = unsafe { ptr.as_ref().props };
        let props = ptr::NonNull::new(props_ptr).map(|ptr| unsafe { ForeignDict::from_ptr(ptr) });

        Self { ptr, props }
    }

    pub fn id(&self) -> u32 {
        unsafe { self.ptr.as_ref().id }
    }

    pub fn change_mask(&self) -> ClientChangeMask {
        let mask = unsafe { self.ptr.as_ref().change_mask };
        ClientChangeMask::from_bits(mask).expect("invalid change_mask")
    }

    pub fn props(&self) -> Option<&ForeignDict> {
        self.props.as_ref()
    }
}

bitflags! {
    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub struct ClientChangeMask: u64 {
        const PROPS = pw_sys::PW_CLIENT_CHANGE_MASK_PROPS as u64;
    }
}

impl fmt::Debug for ClientInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ClientInfo")
            .field("id", &self.id())
            .field("change-mask", &self.change_mask())
            .field("props", &self.props())
            .finish()
    }
}

pub struct ClientListener {
    // Need to stay allocated while the listener is registered
    #[allow(dead_code)]
    events: Pin<Box<pw_sys::pw_client_events>>,
    listener: Pin<Box<spa_sys::spa_hook>>,
    #[allow(dead_code)]
    data: Box<ListenerLocalCallbacks>,
}

impl Listener for ClientListener {}

impl Drop for ClientListener {
    fn drop(&mut self) {
        spa::hook::remove(*self.listener);
    }
}

impl<'a> ClientListenerLocalBuilder<'a> {
    #[must_use]
    pub fn info<F>(mut self, info: F) -> Self
    where
        F: Fn(&ClientInfo) + 'static,
    {
        self.cbs.info = Some(Box::new(info));
        self
    }

    pub fn permissions<F>(mut self, permissions: F) -> Self
    where
        F: Fn(u32, &[Permissions]) + 'static,
    {
        self.cbs.permissions = Some(Box::new(permissions));
        self
    }

    #[must_use]
    pub fn register(self) -> ClientListener {
        unsafe extern "C" fn client_events_info(
            data: *mut c_void,
            info: *const pw_sys::pw_client_info,
        ) {
            let callbacks = (data as *mut ListenerLocalCallbacks).as_ref().unwrap();
            let info = ptr::NonNull::new(info as *mut _).expect("info is NULL");
            let info = ClientInfo::new(info);
            callbacks.info.as_ref().unwrap()(&info);
        }

        unsafe extern "C" fn client_events_permissions(
            data: *mut c_void,
            index: u32,
            n_permissions: u32,
            permissions: *const pw_sys::pw_permission,
        ) {
            let callbacks = (data as *mut ListenerLocalCallbacks).as_ref().unwrap();
            let permissions =
                std::slice::from_raw_parts(permissions.cast(), n_permissions as usize);

            callbacks.permissions.as_ref().unwrap()(index, permissions);
        }

        let e = unsafe {
            let mut e: Pin<Box<pw_sys::pw_client_events>> = Box::pin(mem::zeroed());
            e.version = pw_sys::PW_VERSION_CLIENT_EVENTS;

            if self.cbs.info.is_some() {
                e.info = Some(client_events_info);
            }
            if self.cbs.permissions.is_some() {
                e.permissions = Some(client_events_permissions);
            }

            e
        };

        let (listener, data) = unsafe {
            let client = &self.client.proxy.as_ptr();

            let data = Box::into_raw(Box::new(self.cbs));
            let mut listener: Pin<Box<spa_sys::spa_hook>> = Box::pin(mem::zeroed());
            let listener_ptr: *mut spa_sys::spa_hook = listener.as_mut().get_unchecked_mut();

            spa_interface_call_method!(
                client,
                pw_sys::pw_client_methods,
                add_listener,
                listener_ptr.cast(),
                e.as_ref().get_ref(),
                data as *mut _
            );

            (listener, Box::from_raw(data))
        };

        ClientListener {
            events: e,
            listener,
            data,
        }
    }
}
