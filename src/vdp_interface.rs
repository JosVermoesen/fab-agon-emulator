use std::path::Path;

#[allow(non_snake_case)]
pub struct VdpInterface {
    pub vdp_setup: libloading::Symbol<'static, unsafe extern "C" fn() -> ()>,
    pub vdp_loop: libloading::Symbol<'static, unsafe extern "C" fn()>,
    pub signal_vblank: libloading::Symbol<'static, unsafe extern "C" fn() -> ()>,
    pub copyVgaFramebuffer: libloading::Symbol<
        'static,
        unsafe extern "C" fn(outWidth: *mut u32, outHeight: *mut u32, buffer: *mut u8),
    >,
    pub set_startup_screen_mode: libloading::Symbol<'static, unsafe extern "C" fn(m: u32)>,
    pub z80_send_to_vdp: libloading::Symbol<'static, unsafe extern "C" fn(b: u8)>,
    pub z80_recv_from_vdp: libloading::Symbol<'static, unsafe extern "C" fn(out: *mut u8) -> bool>,
    pub sendHostKbEventToFabgl:
        libloading::Symbol<'static, unsafe extern "C" fn(ps2scancode: u16, isDown: u8)>,
    pub sendHostMouseEventToFabgl:
        libloading::Symbol<'static, unsafe extern "C" fn(mouse_packet: *const u8)>,
    pub setVdpDebugLogging: libloading::Symbol<'static, unsafe extern "C" fn(state: bool) -> ()>,
    pub getAudioSamples:
        libloading::Symbol<'static, unsafe extern "C" fn(out: *mut u8, length: u32)>,
    pub dump_vdp_mem_stats: libloading::Symbol<'static, unsafe extern "C" fn()>,
    pub vdp_shutdown: libloading::Symbol<'static, unsafe extern "C" fn()>,
}

impl VdpInterface {
    fn new(lib: &'static libloading::Library) -> Self {
        unsafe {
            return VdpInterface {
                vdp_setup: lib.get(b"vdp_setup").unwrap(),
                vdp_loop: lib.get(b"vdp_loop").unwrap(),
                signal_vblank: lib.get(b"signal_vblank").unwrap(),
                copyVgaFramebuffer: lib.get(b"copyVgaFramebuffer").unwrap(),
                z80_send_to_vdp: lib.get(b"z80_send_to_vdp").unwrap(),
                z80_recv_from_vdp: lib.get(b"z80_recv_from_vdp").unwrap(),
                set_startup_screen_mode: lib.get(b"set_startup_screen_mode").unwrap(),
                sendHostKbEventToFabgl: lib.get(b"sendHostKbEventToFabgl").unwrap(),
                sendHostMouseEventToFabgl: lib.get(b"sendHostMouseEventToFabgl").unwrap(),
                setVdpDebugLogging: lib.get(b"setVdpDebugLogging").unwrap(),
                getAudioSamples: lib.get(b"getAudioSamples").unwrap(),
                dump_vdp_mem_stats: lib.get(b"dump_vdp_mem_stats").unwrap(),
                vdp_shutdown: lib.get(b"vdp_shutdown").unwrap(),
            };
        }
    }
}

pub fn init(default_vdp: std::path::PathBuf, args: &crate::parse_args::AppArgs) -> VdpInterface {
    assert!(unsafe { VDP_DLL == std::ptr::null() });

    let vdp_dll_path = match args.vdp_dll {
        Some(ref p) => Path::new(".").join(p),
        None => default_vdp,
    };

    if args.verbose {
        eprintln!("VDP firmware: {:?}", vdp_dll_path);
    }

    unsafe {
        VDP_DLL = Box::leak(Box::new(libloading::Library::new(vdp_dll_path).unwrap()));
    }
    VdpInterface::new(unsafe { VDP_DLL.as_ref() }.unwrap())
}
static mut VDP_DLL: *const libloading::Library = std::ptr::null();
