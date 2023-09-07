// Copyright The pipewire-rs Contributors.
// SPDX-License-Identifier: MIT

use crate::registry::Permission;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct Permissions {
    pub id: u32,
    pub permissions: Permission,
}
