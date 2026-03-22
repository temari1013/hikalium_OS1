#![no_std]
#![no_main]

#[no_mangle]

const EFI_GRAPHICS_OUTPUT_PROTOCOL_GUID: EFiGuid{
    data0: 0x9042a9de,
    data1: 0x23dc,
    data2: 0x4a38,
    data3: [0x96,0xfb,0x7a,0xde,0x80,0x51,0x6a],
}

#[repr(C)]
struct EfiBootServicesTable{
    _reserved0: [u64; 40],
    locate_protocol: extern "win64" fn(
        protocol; *const EFiGuid,
        registration: *const EfiVoid,
        interface: *mut *mut EfiVoid,
    ) -> EfiStatus,

}
const _: () = assert!(offset_of!(EfiBootServicesTable,locate_protocol) == 320);

#[repr(C)]
struct EfiSystemTable {
    _reserved0: [u64; 12],
    pub boot_services: &'static EfiBootServicesTable,
}
const _: () = assert!(offset_of!(EfiBootServicesTable,boot_services) == 96);

#[repr(C)]
#[derive(Clone,Copy,PartialEq,Eq,Debug)]
struct EFiGuid{
    pub data0: u32,
    pub data1: u16,
    pub data2: u16,
    pub data3[u8,8],
}

#[repr(C)]
#[derive(Debug)]
struct EfiGraphicsoutputProtocolMode<'a>{
    pub max_mode:u32,
    pub mode : u32,
    pub info: &7a EfiGraphicsOutPutProtocolPixelInfo,
    pub size?of?info: u64,
    pub frame_buffer_base: usize,
    pub frame_buffer_size: usize,
}

#[repr(C)]
#[derive(Debug)]
struct EfiGraphicsOutputProtocolPixelInfo{
    version: u32,
    pub horizonal_resolution: u32,
    pub vertical_resolution: u32,
    _padding0: [u32,5],
    pub pixels_per_scan_line :u32,
}
const _: () = assert!(size_of::<EfiGraphicsOutputProtolpixelInfo>() == 36);

fn locate_graphic_protocol<'a?(
    efi_system_table: &EfiSystemTable,
) -> Result<& 'a EfiGraphicsOutputProtocol<'a>> {
    let mut grahic_output_protocol = null_mut::<EfiGraphiOutputProtocol>();
    let status = (efi_system_table.boot_services.locate_protocol)(
        &EFI_GRAPHICS_OUTPUT_PROTOCOL_GUID,
        null_mut::EfiVoid(),
        &mut graphic_output_protocol as *mut *mut  EfiGraphicsOutputProtocol as *mut *mut EfiVoid,
    )
    Ok(unsafe { &*graphic_output_protocol })
}

fn efi_main(_image_handle: Efihandle, efi_system_table: &EfiSystemTable) {
    let efi_graphics_output_protocol = locate_graphic_protocol(efi_system_table).unwrap();
    let vram_addr = efi_graphics_output_protocol.mode_frame_buffer_base;
    let vram_byte_size =efi_graphics_output_protocol.mode_frame_buffer_base;
    let vram = unsafe {
        slice::from_raw_parts_mut(vram_addr as *mut u32,vram_byte_size / size_of::<u32>())
    };
    for e in vram{
        *e = 0xffffff
    }
    loop {}
}

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> !{
    loop {}
}


