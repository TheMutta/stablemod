#![no_std]

pub mod tree;

#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub mod c_abi {
    include!(concat!(env!("OUT_DIR"), "/abi.rs"));
}

pub type CapabilityHandle = crate::c_abi::capability_handle;
pub type ResourceCapability = crate::c_abi::resource_capability;
pub type VirtualCapability = crate::c_abi::virtual_capability;
pub type ResourceCapabilityArena = crate::c_abi::resource_capability_arena;
pub type VirtualCapabilityArena = crate::c_abi::virtual_capability_arena;

pub const SYS_DEBUG: usize = 0;

impl CapabilityHandle {
    pub fn new(generation_id: u64, arena_id: u64) -> Self {
        Self {
            generation_id,
            arena_id,
        }
    }
}

impl ResourceCapability {
    pub fn new(permissions: u64, resource: u64, size: u64, genid: u64, derived_refs: u64, virtual_refs: u64) -> Self {
        Self {
            permissions,
            resource,
            size,
            genid,
            derived_refs,
            virtual_refs,
        }
    }
}

impl VirtualCapability {
    pub fn new(permissions: u64, resource: u64, size: u64, genid: u64, rescap: CapabilityHandle) -> Self {
        Self {
            permissions,
            resource,
            size,
            genid,
            rescap,
        }
    }
}

impl ResourceCapabilityArena {
    pub fn new(arenaid: u64, slots: u32, slots_free: u32, arena_cap: ResourceCapability) -> Self{
        Self {
            arenaid,
            slots,
            slots_free,
            arena_cap,
            ..Default::default()
        }
    }

    pub fn get_slots(&self) -> &[ResourceCapability] {
        unsafe {
            let ptr = &self.res_cap  as *const _ as *const ResourceCapability;
            let slice: &[ResourceCapability] = core::slice::from_raw_parts(ptr, self.slots as usize);

            slice
        }
    }

    pub fn get_slots_ptr(&mut self) -> *mut ResourceCapability {
        let ptr = &self.res_cap  as *const _ as *const ResourceCapability as *mut _;
        ptr
    }
}

impl VirtualCapabilityArena {
    pub fn new(arenaid: u64, slots: u32, slots_free: u32, arena_cap: ResourceCapability) -> Self {
        Self {
            arenaid,
            slots,
            slots_free,
            arena_cap,
            ..Default::default()
        }
    }

    pub fn get_slots(&self) -> &[VirtualCapability] {
        unsafe {
            let ptr = &self.virt_cap  as *const _ as *const VirtualCapability;
            let slice: &[VirtualCapability] = core::slice::from_raw_parts(ptr, self.slots as usize);

            slice
        }
    }
}
