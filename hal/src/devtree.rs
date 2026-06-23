


pub struct HalDeviceTree {

}

impl HalDeviceTree {
    pub fn from_rsdp(rsdp: u64) {

    }
}

pub struct HalBus {

}

pub struct HalDevice {

}

pub struct HalInterface {

}


#[repr(C)]
struct FDTHeader {
    /// This field shall contain the value 0xd00dfeed (big-endian).
    magic: u32,

    /// This field shall contain the total size in bytes of the devicetree data structure.
    /// This size shall encompass all sections of the structure:
    ///  - the header,
    ///  - the memory reservation block,
    ///  - structure block and strings block,
    ///  - as well as any free space gaps between the blocks or after the final block.
    total_size: u32,

    /// This field shall contain the offset in bytes of the structure block (see section 5.4) from the beginning of the header.
    off_dt_struct: u32,

    /// This field shall contain the offset in bytes of the strings block (see section 5.5) from the beginning of the header. 
    off_dt_strings: u32,

    /// This field shall contain the offset in bytes of the memory reservation block (see section 5.3) from the beginning of the header.
    off_mem_rsvmap: u32,

    /// This field shall contain the version of the devicetree data structure.
    /// The version is 17 if using the structure as defined in this document.
    /// An DTSpec boot program may provide the devicetree of a later version, in which case this field shall contain the version number defined in whichever later document gives the details of that version.
    version: u32,

    /// This field shall contain the lowest version of the devicetree data structure with which the version used is backwards compatible. So, for the structure as defined in this document (version 17), this field shall contain 16 because version 17 is backwards compatible with version 16, but not earlier versions. As per section 5.1, a DTSpec boot program should provide a devicetree in a format which is backwards compatible with version 16, and thus this field shall always contain 16.
    last_comp_version: u32,

    /// This field shall contain the physical ID of the system’s boot CPU. It shall be identical to the physical ID given in the reg property of that CPU node within the devicetree.
    boot_cpuid_phys: u32,

    /// This field shall contain the length in bytes of the strings block section of the devicetree blob.
    size_dt_strings: u32,

    /// This field shall contain the length in bytes of the structure block section of the devicetree blob.
    size_dt_struct: u32,
}

#[repr(C)]
struct FDTReserveEntry {
    address: u64,
    size: u64,
}

/// The FDT_BEGIN_NODE token marks the beginning of a node’s representation. It shall be followed by the node’s unit name as extra data. The name is stored as a null-terminated string, and shall include the unit address (see section 2.2.1), if any. The node name is followed by zeroed padding bytes, if necessary for alignment, and then the next token, which may be any token except FDT_END.
const FDT_BEGIN_NODE: u32 = 0x00000001;

/// The FDT_END_NODE token marks the end of a node’s representation. This token has no extra data; so it is followed immediately by the next token, which may be any token except FDT_PROP.
const FDT_END_NODE: u32 = 0x00000002;

/// The FDT_PROP token marks the beginning of the representation of one property in the devicetree. It shall be followed by extra data describing the property. This data consists first of the property’s length and name represented as the following C structure:
const FDT_PROP: u32 = 0x00000003;
#[repr(C)]
struct FDTProp {
    /// gives the length of the property’s value in bytes (which may be zero, indicating an empty property, see section 2.2.4.2).
    len: u32,

    /// gives an offset into the strings block (see section 5.5) at which the property’s name is stored as a null-terminated string.
    nameoff: u32,
}

/// The FDT_NOP token will be ignored by any program parsing the device tree. This token has no extra data; so it is followed immediately by the next token, which can be any valid token. A property or node definition in the tree can be overwritten with FDT_NOP tokens to remove it from the tree without needing to move other sections of the tree’s representation in the devicetree blob.
const FDT_NOP: u32 = 0x00000004;

/// The FDT_END token marks the end of the structure block. There shall be only one FDT_END token, and it shall be the last token in the structure block. It has no extra data; so the byte immediately after the FDT_END token has offset from the beginning of the structure block equal to the value of the size_dt_struct field in the device tree blob header.
const FDT_END: u32 = 0x00000009;

