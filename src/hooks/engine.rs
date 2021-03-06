//! `hw`, `sw`, `hl`.

#![allow(non_snake_case, non_upper_case_globals)]

use std::{
    ffi::CString,
    os::raw::*,
    ptr::{null_mut, NonNull},
};

use bxt_macros::pattern;
use bxt_patterns::Patterns;
use rust_hawktracer::*;

use crate::{
    ffi::{command::cmd_function_s, cvar::cvar_s, playermove::playermove_s, usercmd::usercmd_s},
    hooks::{sdl, server},
    modules::{
        capture, commands, cvars, demo_playback, fade_remove, force_fov, shake_remove, tas_logging,
    },
    utils::*,
    vulkan,
};

pub static build_number: Pointer<unsafe extern "C" fn() -> c_int> = Pointer::empty_patterns(
    b"build_number\0",
    // To find, search for "Half-Life %i/%s (hw build %d)". This function is
    // Draw_ConsoleBackground(), and a call to build_number() is right above the snprintf() using
    // this string.
    Patterns(&[
        // 6153
        pattern!(55 8B EC 83 EC 08 A1 ?? ?? ?? ?? 56 33 F6 85 C0),
        // 4554
        pattern!(A1 ?? ?? ?? ?? 83 EC 08 57 33 FF 85 C0 0F 85 A5 00 00 00 53 56 33 DB BE ?? ?? ?? ?? 8B 06 8B 0D),
    ]),
    null_mut(),
);
pub static Cbuf_InsertText: Pointer<unsafe extern "C" fn(*const c_char)> =
    Pointer::empty(b"Cbuf_InsertText\0");
pub static CL_Disconnect: Pointer<unsafe extern "C" fn()> = Pointer::empty_patterns(
    b"CL_Disconnect\0",
    // To find, search for "ExitGame".
    Patterns(&[
        // 6153
        pattern!(55 8B EC 83 EC 14 53 56 33 DB),
    ]),
    my_CL_Disconnect as _,
);
pub static CL_GameDir_f: Pointer<unsafe extern "C" fn()> = Pointer::empty_patterns(
    b"CL_GameDir_f\0",
    // To find, search for "gamedir is ".
    Patterns(&[
        // 6153
        pattern!(E8 ?? ?? ?? ?? 83 F8 02 74 ?? 68 ?? ?? ?? ?? 68),
    ]),
    null_mut(),
);
pub static cls: Pointer<*mut client_static_s> = Pointer::empty(b"cls\0");
pub static cls_demos: Pointer<*mut client_static_s_demos> = Pointer::empty(
    // Not a real symbol name.
    b"cls_demos\0",
);
pub static Cmd_AddMallocCommand: Pointer<
    unsafe extern "C" fn(*const c_char, unsafe extern "C" fn(), c_int),
> = Pointer::empty_patterns(
    b"Cmd_AddMallocCommand\0",
    // To find, search for "Cmd_AddCommand: %s already defined as a var". It will give two results,
    // one of them for Cmd_AddCommandWithFlags, another for Cmd_AddMallocCommand.
    // Cmd_AddMallocCommand is slightly smaller, and the allocation call in the middle that takes
    // 0x10 as a parameter calls malloc internally. This allocation call is Mem_ZeroMalloc.
    Patterns(&[
        // 6153
        pattern!(55 8B EC 56 57 8B 7D ?? 57 E8 ?? ?? ?? ?? 8A 08),
        // 4554
        pattern!(56 57 8B 7C 24 ?? 57 E8 ?? ?? ?? ?? 8A 08),
    ]),
    null_mut(),
);
pub static Cmd_Argc: Pointer<unsafe extern "C" fn() -> c_int> = Pointer::empty(b"Cmd_Argc\0");
pub static Cmd_Argv: Pointer<unsafe extern "C" fn(c_int) -> *const c_char> =
    Pointer::empty(b"Cmd_Argv\0");
pub static cmd_functions: Pointer<*mut *mut cmd_function_s> = Pointer::empty(b"cmd_functions\0");
pub static Con_Printf: Pointer<unsafe extern "C" fn(*const c_char, ...)> = Pointer::empty_patterns(
    b"Con_Printf\0",
    // To find, search for "qconsole.log". One of the three usages is Con_Printf (the one that
    // isn't just many function calls or OutputDebugStringA).
    Patterns(&[
        // 6153
        pattern!(55 8B EC B8 00 10 00 00 E8 ?? ?? ?? ?? 8B 4D),
        // 4554
        pattern!(B8 00 10 00 00 E8 ?? ?? ?? ?? 8B 8C 24),
    ]),
    null_mut(),
);
pub static Con_ToggleConsole_f: Pointer<unsafe extern "C" fn()> = Pointer::empty_patterns(
    b"Con_ToggleConsole_f\0",
    // To find, search for "toggleconsole". Look for console command registration, the callback will
    // be Con_ToggleConsole_f().
    Patterns(&[
        // 6153
        pattern!(E8 ?? ?? ?? ?? 85 C0 74 ?? E9 ?? ?? ?? ?? E9),
    ]),
    my_Con_ToggleConsole_f as _,
);
pub static com_gamedir: Pointer<*mut [c_char; 260]> = Pointer::empty(b"com_gamedir\0");
pub static Cvar_RegisterVariable: Pointer<unsafe extern "C" fn(*mut cvar_s)> =
    Pointer::empty_patterns(
        b"Cvar_RegisterVariable\0",
        // To find, search for "Can't register variable %s, already defined".
        Patterns(&[
            // 6153
            pattern!(55 8B EC 83 EC 14 53 56 8B 75 ?? 57 8B 06),
            // 4554
            pattern!(83 EC 14 53 56 8B 74 24),
        ]),
        null_mut(),
    );
pub static cvar_vars: Pointer<*mut *mut cvar_s> = Pointer::empty(b"cvar_vars\0");
pub static GL_BeginRendering: Pointer<
    unsafe extern "C" fn(*mut c_int, *mut c_int, *mut c_int, *mut c_int),
> = Pointer::empty_patterns(
    b"GL_BeginRendering\0",
    // To find, take usages of glClear(). The shortest is GL_BeginRendering().
    Patterns(&[
        // 6153
        pattern!(55 8B EC 8B 45 ?? 8B 4D ?? 56 57),
    ]),
    null_mut(),
);
pub static gEntityInterface: Pointer<*mut DllFunctions> = Pointer::empty(b"gEntityInterface\0");
pub static Key_Event: Pointer<unsafe extern "C" fn(c_int, c_int)> = Pointer::empty_patterns(
    b"Key_Event\0",
    // To find, search for "ctrl-alt-del pressed".
    Patterns(&[
        // 6153
        pattern!(55 8B EC 81 EC 00 04 00 00 8B 45 ?? 56 3D 00 01 00 00),
        // 4554
        pattern!(81 EC 00 04 00 00 8D 84 24 ?? ?? ?? ?? 8D 8C 24),
    ]),
    my_Key_Event as _,
);
pub static LoadEntityDLLs: Pointer<unsafe extern "C" fn(*const c_char)> = Pointer::empty_patterns(
    b"LoadEntityDLLs\0",
    // To find, search for "GetNewDLLFunctions".
    Patterns(&[
        // 6153
        // Don't use this for com_gamedir as the pattern matches versions with different offsets.
        pattern!(55 8B EC B8 90 23 00 00),
        // 4554
        pattern!(81 EC 94 04 00 00 53 56 E8),
    ]),
    my_LoadEntityDLLs as _,
);
pub static Host_FilterTime: Pointer<unsafe extern "C" fn(c_float) -> c_int> =
    Pointer::empty_patterns(
        b"Host_FilterTime\0",
        // To find, search for "-sys_ticrate". The parent will be _Host_Frame().
        Patterns(&[
            // 6153
            pattern!(55 8B EC 83 EC 08 D9 05 ?? ?? ?? ?? D8 1D),
            // 4554
            pattern!(55 8B EC 83 E4 F8 83 EC 08 D9 05 ?? ?? ?? ?? D8 1D ?? ?? ?? ?? DF E0 F6 C4 41),
        ]),
        my_Host_FilterTime as _,
    );
pub static host_frametime: Pointer<*mut c_double> = Pointer::empty(b"host_frametime\0");
pub static Host_InitializeGameDLL: Pointer<unsafe extern "C" fn()> = Pointer::empty_patterns(
    b"Host_InitializeGameDLL\0",
    // To find, search for "Sys_InitializeGameDLL called twice, skipping second call".
    // Alternatively, find LoadEntityDLLs() and go to the parent function.
    Patterns(&[
        // 6153
        pattern!(E8 ?? ?? ?? ?? 8B 0D ?? ?? ?? ?? 33 C0 83 F9 01),
    ]),
    null_mut(),
);
pub static Host_NextDemo: Pointer<unsafe extern "C" fn()> = Pointer::empty_patterns(
    b"Host_NextDemo\0",
    // To find, search for "No demos listed with startdemos".
    Patterns(&[
        // 6153
        pattern!(55 8B EC 81 EC 00 04 00 00 83 3D ?? ?? ?? ?? FF),
        // 4554
        pattern!(A1 ?? ?? ?? ?? 81 EC 00 04 00 00 83 F8 FF),
    ]),
    my_Host_NextDemo as _,
);
pub static Host_Shutdown: Pointer<unsafe extern "C" fn()> = Pointer::empty_patterns(
    b"Host_Shutdown\0",
    // To find, search for "recursive shutdown".
    Patterns(&[
        // 6153
        pattern!(A1 ?? ?? ?? ?? 53 33 DB 3B C3 74 ?? 68),
    ]),
    my_Host_Shutdown as _,
);
pub static Host_Tell_f: Pointer<unsafe extern "C" fn()> = Pointer::empty_patterns(
    b"Host_Tell_f\0",
    // To find, search for "%s TELL: ".
    Patterns(&[
        // 6153
        pattern!(55 8B EC 83 EC 40 A1 ?? ?? ?? ?? 56),
        // 4554
        pattern!(A1 ?? ?? ?? ?? 83 EC 40 83 F8 01 56 75 0A E8),
    ]),
    null_mut(),
);
pub static Host_ValidSave: Pointer<unsafe extern "C" fn() -> c_int> = Pointer::empty_patterns(
    b"Host_ValidSave\0",
    // To find, search for "Not playing a local game.".
    Patterns(&[
        // 6153
        pattern!(A1 ?? ?? ?? ?? B9 01 00 00 00 3B C1 0F 85),
    ]),
    null_mut(),
);
pub static Memory_Init: Pointer<unsafe extern "C" fn(*mut c_void, c_int) -> c_int> =
    Pointer::empty_patterns(
        b"Memory_Init\0",
        // To find, search for "Memory_Init".
        Patterns(&[
            // 6153
            pattern!(55 8B EC 8B 45 ?? 8B 4D ?? 56 BE 00 00 20 00),
            // 4554
            pattern!(8B 44 24 ?? 8B 4C 24 ?? 56 BE 00 00 20 00),
        ]),
        my_Memory_Init as _,
    );
pub static Mem_Free: Pointer<unsafe extern "C" fn(*mut c_void)> = Pointer::empty_patterns(
    b"Mem_Free\0",
    // Mem_Free is called once in Host_Shutdown to free a pointer after checking that it's != 0. On
    // Windows, it dispatches directly to an underlying function, and the pattern is for the
    // underlying function.
    Patterns(&[
        // 6153
        pattern!(55 8B EC 6A FF 68 ?? ?? ?? ?? 68 ?? ?? ?? ?? 64 A1 ?? ?? ?? ?? 50 64 89 25 ?? ?? ?? ?? 83 EC 18 53 56 57 8B 75 ?? 85 F6),
        // 4554
        pattern!(56 8B 74 24 ?? 85 F6 74 ?? 6A 09),
    ]),
    null_mut(),
);
pub static paintbuffer: Pointer<*mut [portable_samplepair_t; 1026]> =
    Pointer::empty(b"paintbuffer\0");
pub static paintedtime: Pointer<*mut c_int> = Pointer::empty(b"paintedtime\0");
pub static realtime: Pointer<*mut f64> = Pointer::empty(b"realtime\0");
pub static R_SetFrustum: Pointer<unsafe extern "C" fn()> = Pointer::empty_patterns(
    b"R_SetFrustum\0",
    // To find, search for "R_RenderView". This is R_RenderView(). The call between two if (global
    // == 0) {} conditions is R_RenderScene(). Open R_RenderScene(). The second call after the first
    // check is R_SetFrustum().
    Patterns(&[
        // 6153
        pattern!(55 8B EC 83 EC 08 DB 05),
        // 4554
        pattern!(83 EC 08 DB 05 ?? ?? ?? ?? A1 ?? ?? ?? ?? 56 89 44 24 04),
    ]),
    my_R_SetFrustum as _,
);
pub static ReleaseEntityDlls: Pointer<unsafe extern "C" fn()> = Pointer::empty_patterns(
    b"ReleaseEntityDlls\0",
    // Find Host_Shutdown(). It has a Mem_Free() if. The 3-rd function above that if is
    // ReleaseEntityDlls().
    Patterns(&[
        // 6153
        pattern!(A1 ?? ?? ?? ?? 56 57 BE ?? ?? ?? ?? 8D 04),
    ]),
    my_ReleaseEntityDlls as _,
);
pub static S_PaintChannels: Pointer<unsafe extern "C" fn(c_int)> = Pointer::empty_patterns(
    b"S_PaintChannels\0",
    // To find, search for "Start profiling 10,000 calls to DSP". This is S_Say(). A call below
    // which has an argument of something + 0x4e2000 is S_PaintChannels().
    Patterns(&[
        // 6153
        pattern!(55 8B EC A1 ?? ?? ?? ?? 53 8B 5D ?? 3B C3 0F 8D),
    ]),
    my_S_PaintChannels as _,
);
pub static S_TransferStereo16: Pointer<unsafe extern "C" fn(c_int)> = Pointer::empty_patterns(
    b"S_TransferStereo16\0",
    // To find, find S_PaintChannels(), go into the last call before the while () condition in the
    // end and this will be the function that that one falls through into. Alternatively, search for
    // "S_TransferStereo16".
    Patterns(&[
        // 6153
        pattern!(55 8B EC 83 EC 0C D9 05 ?? ?? ?? ?? D8 0D),
    ]),
    my_S_TransferStereo16 as _,
);
pub static scr_fov_value: Pointer<*mut c_float> = Pointer::empty(b"scr_fov_value\0");
pub static shm: Pointer<*mut *mut dma_t> = Pointer::empty(b"shm\0");
pub static sv: Pointer<*mut c_void> = Pointer::empty(b"sv\0");
pub static SV_Frame: Pointer<unsafe extern "C" fn()> = Pointer::empty_patterns(
    b"SV_Frame\0",
    // To find, search for "%s timed out". It is used in SV_CheckTimeouts(), which is called by
    // SV_Frame().
    Patterns(&[
        // 6153
        pattern!(A1 ?? ?? ?? ?? 85 C0 74 ?? DD 05 ?? ?? ?? ?? A1),
    ]),
    my_SV_Frame as _,
);
pub static Sys_VID_FlipScreen: Pointer<unsafe extern "C" fn()> = Pointer::empty_patterns(
    b"_Z18Sys_VID_FlipScreenv\0",
    // To find, search for "Sys_InitLauncherInterface()". Go into function right after the one that
    // accepts this string as an argument. The last function pointer assigned is
    // Sys_VID_FlipScreen(). It checks one pointer for NULL then calls SDL_GL_SwapWindow().
    Patterns(&[
        // 6153
        pattern!(A1 ?? ?? ?? ?? 85 C0 74 ?? 8B 00),
    ]),
    my_Sys_VID_FlipScreen as _,
);
pub static V_ApplyShake: Pointer<unsafe extern "C" fn(*mut c_float, *mut c_float, c_float)> =
    Pointer::empty_patterns(
        b"V_ApplyShake\0",
        // To find, search for "ScreenShake". This is ClientDLL_Init(), near the bottom there are
        // two similar function calls, one is using our string as the 1st param and the 2nd param as
        // another function, open it. This is V_ScreenShake(), right above it is V_ApplyShake().
        Patterns(&[
            // 6153
            pattern!(55 8B EC 8D 45 ?? 8D 4D ?? 50 8D 55 ?? 51 52 FF 15 ?? ?? ?? ?? 8B 45 ?? 83 C4 0C),
            // 4554
            pattern!(8D 44 24 ?? 8D 4C 24 ?? 50 8D 54 24 ?? 51 52 FF 15 ?? ?? ?? ?? 8B 44 24 ?? 83 C4 0C),
        ]),
        my_V_ApplyShake as _,
    );
pub static V_FadeAlpha: Pointer<unsafe extern "C" fn() -> c_int> = Pointer::empty_patterns(
    b"V_FadeAlpha\0",
    // To find, search for "%3ifps %3i ms  %4i wpoly %4i epoly". This will lead to either
    // R_RenderView() or its usually-inlined part, and the string will be used within an if. Right
    // above the if is S_ExtraUpdate(), and right above that (maybe in another if) is
    // R_PolyBlend(). Inside R_PolyBlend(), the first call is V_FadeAlpha().
    Patterns(&[
        // 6153
        pattern!(55 8B EC 83 EC 08 D9 05 ?? ?? ?? ?? DC 1D),
        // 4554
        pattern!(D9 05 ?? ?? ?? ?? DC 1D ?? ?? ?? ?? 8A 0D ?? ?? ?? ?? 83 EC 08),
    ]),
    my_V_FadeAlpha as _,
);
pub static VideoMode_IsWindowed: Pointer<unsafe extern "C" fn() -> c_int> = Pointer::empty_patterns(
    b"VideoMode_IsWindowed\0",
    // To find, take usages of glClear(). The shortest is GL_BeginRendering(). The first check is
    // for the return value of VideoMode_IsWindowed().
    Patterns(&[
        // 6153
        pattern!(8B 0D ?? ?? ?? ?? 85 C9 74 ?? 8B 01 FF 50 ?? 84 C0),
    ]),
    null_mut(),
);
pub static VideoMode_GetCurrentVideoMode: Pointer<
    unsafe extern "C" fn(*mut c_int, *mut c_int, *mut c_int),
> = Pointer::empty_patterns(
    b"VideoMode_GetCurrentVideoMode\0",
    // To find, take usages of glClear(). The shortest is GL_BeginRendering(). The first if calls
    // VideoMode_GetCurrentVideoMode().
    Patterns(&[
        // 6153
        pattern!(55 8B EC 8B 0D ?? ?? ?? ?? 8B 01 FF 50 ?? 85 C0),
    ]),
    null_mut(),
);
pub static window_rect: Pointer<*mut Rect> = Pointer::empty(b"window_rect\0");
pub static Z_Free: Pointer<unsafe extern "C" fn(*mut c_void)> = Pointer::empty_patterns(
    b"Z_Free\0",
    // To find, search for "Z_Free: NULL pointer".
    Patterns(&[
        // 6153
        pattern!(55 8B EC 56 8B 75 ?? 85 F6 57 75 ?? 68 ?? ?? ?? ?? E8 ?? ?? ?? ?? 83 C4 04 8B 46),
        // 4554
        pattern!(56 8B 74 24 ?? 85 F6 57 75 ?? 68 ?? ?? ?? ?? E8 ?? ?? ?? ?? 83 C4 04 8B 46),
    ]),
    null_mut(),
);

static POINTERS: &[&dyn PointerTrait] = &[
    &build_number,
    &Cbuf_InsertText,
    &CL_Disconnect,
    &CL_GameDir_f,
    &cls,
    &cls_demos,
    &Cmd_AddMallocCommand,
    &Cmd_Argc,
    &Cmd_Argv,
    &cmd_functions,
    &Con_Printf,
    &Con_ToggleConsole_f,
    &com_gamedir,
    &Cvar_RegisterVariable,
    &cvar_vars,
    &GL_BeginRendering,
    &gEntityInterface,
    #[cfg(not(feature = "bxt-compatibility"))]
    &Key_Event,
    &LoadEntityDLLs,
    &Host_FilterTime,
    &host_frametime,
    &Host_InitializeGameDLL,
    &Host_NextDemo,
    &Host_Shutdown,
    &Host_Tell_f,
    &Host_ValidSave,
    &Memory_Init,
    &Mem_Free,
    &paintbuffer,
    &paintedtime,
    &realtime,
    &R_SetFrustum,
    &ReleaseEntityDlls,
    &S_PaintChannels,
    &S_TransferStereo16,
    &scr_fov_value,
    &shm,
    &sv,
    #[cfg(not(feature = "bxt-compatibility"))]
    &SV_Frame,
    &Sys_VID_FlipScreen,
    &V_ApplyShake,
    #[cfg(not(feature = "bxt-compatibility"))]
    &V_FadeAlpha,
    &VideoMode_IsWindowed,
    &VideoMode_GetCurrentVideoMode,
    &window_rect,
    &Z_Free,
];

#[cfg(windows)]
static ORIGINAL_FUNCTIONS: MainThreadRefCell<Vec<*mut c_void>> = MainThreadRefCell::new(Vec::new());

#[repr(C)]
pub struct DllFunctions {
    _padding_1: [u8; 136],
    pub pm_move: Option<unsafe extern "C" fn(*mut playermove_s, c_int)>,
    _padding_2: [u8; 32],
    pub cmd_start: Option<unsafe extern "C" fn(*mut c_void, *mut usercmd_s, c_uint)>,
}

#[cfg(unix)]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct Rect {
    pub left: i32,
    pub right: i32,
    pub top: i32,
    pub bottom: i32,
}
#[cfg(windows)]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct Rect {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct portable_samplepair_t {
    pub left: c_int,
    pub right: c_int,
}

#[repr(C)]
pub struct dma_t {
    pub gamealive: c_int,
    pub soundalive: c_int,
    pub splitbuffer: c_int,
    pub channels: c_int,
    pub samples: c_int,
    pub submission_chunk: c_int,
    pub samplepos: c_int,
    pub samplebits: c_int,
    pub speed: c_int,
    pub dmaspeed: c_int,
    pub buffer: *mut c_uchar,
}

#[repr(C)]
pub struct client_static_s {
    pub state: c_int,
}

#[repr(C)]
pub struct client_static_s_demos {
    pub demonum: c_int,
    pub demos: [[c_char; 16]; 32],
    pub demorecording: c_int,
    pub demoplayback: c_int,
}

/// Prints the string to the console.
///
/// If `Con_Printf` was not found, does nothing.
///
/// Any null-bytes are replaced with a literal `"\x00"`.
pub fn con_print(marker: MainThreadMarker, s: &str) {
    if !Con_Printf.is_set(marker) {
        return;
    }

    let s = to_cstring_lossy(s);

    // Safety: Con_Printf() uses global buffers which are always valid, and external calls are
    // guarded with other global variables being non-zero, so they cannot be incorrectly called
    // either.
    unsafe {
        Con_Printf.get(marker)(b"%s\0".as_ptr().cast(), s.as_ptr());
    }
}

/// Prepends the command to the engine command buffer.
///
/// If `command` contains null-bytes, up to the first null-byte will be inserted.
///
/// # Panics
///
/// Panics if `Cbuf_InsertText` was not found.
pub fn prepend_command(marker: MainThreadMarker, command: &str) {
    let command = match CString::new(command) {
        Ok(command) => command,
        Err(nul_error) => {
            let nul_position = nul_error.nul_position();
            let mut bytes = nul_error.into_vec();
            bytes.truncate(nul_position);
            CString::new(bytes).unwrap()
        }
    };

    // Safety: Cbuf_InsertText() uses a global buffer which is zeroed by default. It means that
    // before it is initialized its max size equals to 0, which will trigger the error condition in
    // Cbuf_InsertText() early. The error condition calls Con_Printf(), which is also safe (see
    // safety comment in [`con_print()`]).
    unsafe {
        Cbuf_InsertText.get(marker)(command.as_ptr());
    }
}

/// # Safety
///
/// [`reset_pointers()`] must be called before hw is unloaded so the pointers don't go stale.
#[cfg(unix)]
#[hawktracer(find_pointers)]
unsafe fn find_pointers(marker: MainThreadMarker) {
    use libc::{RTLD_NOLOAD, RTLD_NOW};
    use libloading::os::unix::Library;

    let library = Library::open(Some("hw.so"), RTLD_NOW | RTLD_NOLOAD).unwrap();

    for pointer in POINTERS {
        let ptr = library
            .get(pointer.symbol())
            .ok()
            .and_then(|sym| NonNull::new(*sym));
        pointer.set(marker, ptr);
    }

    cls_demos.set(marker, cls.offset(marker, 15960));

    for pointer in POINTERS {
        pointer.log(marker);
    }
}

/// # Safety
///
/// The memory starting at `base` with size `size` must be valid to read and not modified over the
/// duration of this call. If any pointers are found in memory, then the memory must be valid until
/// the pointers are reset (according to the safety section of `PointerTrait::set`).
#[allow(clippy::single_match)]
#[cfg(windows)]
#[hawktracer(find_pointers)]
pub unsafe fn find_pointers(marker: MainThreadMarker, base: *mut c_void, size: usize) {
    use std::slice;

    use minhook_sys::*;

    // Find all pattern-based pointers.
    {
        let memory = slice::from_raw_parts(base.cast(), size);
        for pointer in POINTERS {
            if let Some((offset, index)) = pointer.patterns().find(memory) {
                pointer.set_with_index(
                    marker,
                    NonNull::new_unchecked(base.add(offset)),
                    Some(index),
                );
            }
        }
    }

    // Find all offset-based pointers.
    let ptr = &CL_GameDir_f;
    match ptr.pattern_index(marker) {
        // 6153
        Some(0) => com_gamedir.set(marker, ptr.by_offset(marker, 11)),
        _ => (),
    }

    let ptr = &Cmd_AddMallocCommand;
    match ptr.pattern_index(marker) {
        // 6153
        Some(0) => cmd_functions.set(marker, ptr.by_offset(marker, 43)),
        // 4554
        Some(1) => cmd_functions.set(marker, ptr.by_offset(marker, 40)),
        _ => (),
    }

    let ptr = &Cvar_RegisterVariable;
    match ptr.pattern_index(marker) {
        // 6153
        Some(0) => cvar_vars.set(marker, ptr.by_offset(marker, 124)),
        // 4554
        Some(1) => cvar_vars.set(marker, ptr.by_offset(marker, 122)),
        _ => (),
    }

    let ptr = &Host_InitializeGameDLL;
    match ptr.pattern_index(marker) {
        // 6153
        Some(0) => {
            // svs.set(marker, ptr.by_offset(marker, 26));
            LoadEntityDLLs.set_if_empty(marker, ptr.by_relative_call(marker, 69));
            gEntityInterface.set(marker, ptr.by_offset(marker, 75));
        }
        _ => (),
    }

    let ptr = &Host_FilterTime;
    match ptr.pattern_index(marker) {
        // 6153
        Some(0) => {
            host_frametime.set(marker, ptr.by_offset(marker, 64));
            realtime.set(marker, ptr.by_offset(marker, 70));
        }
        // 4554
        Some(1) => {
            host_frametime.set(marker, ptr.by_offset(marker, 65));
            realtime.set(marker, ptr.by_offset(marker, 71));
        }
        _ => (),
    }

    let ptr = &Host_NextDemo;
    match ptr.pattern_index(marker) {
        // 6153
        Some(0) => {
            Cbuf_InsertText.set(marker, ptr.by_relative_call(marker, 140));
            cls_demos.set(marker, ptr.by_offset(marker, 11));
        }
        // 4554
        Some(1) => {
            Cbuf_InsertText.set(marker, ptr.by_relative_call(marker, 137));
            cls_demos.set(marker, ptr.by_offset(marker, 1));
        }
        _ => (),
    }

    let ptr = &Host_Tell_f;
    match ptr.pattern_index(marker) {
        // 6153
        Some(0) => {
            Cmd_Argc.set(marker, ptr.by_relative_call(marker, 28));
            Cmd_Argv.set(marker, ptr.by_relative_call(marker, 145));
        }
        // 4554
        Some(1) => {
            Cmd_Argc.set(marker, ptr.by_relative_call(marker, 25));
            Cmd_Argv.set(marker, ptr.by_relative_call(marker, 144));
        }
        _ => (),
    }

    let ptr = &Host_ValidSave;
    match ptr.pattern_index(marker) {
        // 6153
        Some(0) => {
            sv.set(marker, ptr.by_offset(marker, 19));
            cls.set(marker, ptr.by_offset(marker, 69));
            Con_Printf.set_if_empty(marker, ptr.by_relative_call(marker, 33));
        }
        _ => (),
    }

    let ptr = &GL_BeginRendering;
    match ptr.pattern_index(marker) {
        // 6153
        Some(0) => {
            VideoMode_IsWindowed.set_if_empty(marker, ptr.by_relative_call(marker, 24));
            VideoMode_GetCurrentVideoMode.set_if_empty(marker, ptr.by_relative_call(marker, 79));
            window_rect.set(marker, ptr.by_offset(marker, 43));
        }
        _ => (),
    }

    let ptr = &R_SetFrustum;
    match ptr.pattern_index(marker) {
        // 6153
        Some(0) => {
            scr_fov_value.set(marker, ptr.by_offset(marker, 13));
        }
        // 4554
        Some(1) => {
            scr_fov_value.set(marker, ptr.by_offset(marker, 10));
        }
        _ => (),
    }

    let ptr = &ReleaseEntityDlls;
    match ptr.pattern_index(marker) {
        // 6153
        Some(0) => {
            // svs.set(marker, ptr.by_offset(marker, 23));
        }
        _ => (),
    }

    let ptr = &S_PaintChannels;
    match ptr.pattern_index(marker) {
        // 6153
        Some(0) => {
            paintedtime.set(marker, ptr.by_offset(marker, 4));
            paintbuffer.set(marker, ptr.by_offset(marker, 60));
        }
        _ => (),
    }

    let ptr = &S_TransferStereo16;
    match ptr.pattern_index(marker) {
        // 6153
        Some(0) => {
            shm.set(marker, ptr.by_offset(marker, 337));
        }
        _ => (),
    }

    let ptr = &SV_Frame;
    match ptr.pattern_index(marker) {
        // 6153
        Some(0) => {
            sv.set(marker, ptr.by_offset(marker, 1));
            host_frametime.set(marker, ptr.by_offset(marker, 11));
        }
        _ => (),
    }

    // Hook all found pointers.
    for pointer in POINTERS {
        pointer.log(marker);

        if !pointer.is_set(marker) {
            continue;
        }

        let hook_fn = pointer.hook_fn();
        if hook_fn.is_null() {
            continue;
        }

        let original = pointer.get_raw(marker);
        let mut trampoline = null_mut();
        assert!(MH_CreateHook(original.as_ptr(), hook_fn, &mut trampoline,) == MH_OK);

        // Store the original pointer to be able to remove the hook later.
        ORIGINAL_FUNCTIONS
            .borrow_mut(marker)
            .push(original.as_ptr());

        // Store the trampoline pointer which is used to call the original function.
        pointer.set_with_index(
            marker,
            NonNull::new_unchecked(trampoline),
            pointer.pattern_index(marker),
        );

        assert!(MH_EnableHook(original.as_ptr()) == MH_OK);
    }
}

fn reset_pointers(marker: MainThreadMarker) {
    for pointer in POINTERS {
        pointer.reset(marker);
    }

    // Remove all hooks.
    #[cfg(windows)]
    {
        use minhook_sys::*;

        for function in ORIGINAL_FUNCTIONS.borrow_mut(marker).drain(..) {
            assert!(unsafe { MH_RemoveHook(function) } == MH_OK);
        }
    }
}

use exported::*;

/// Functions exported for `LD_PRELOAD` hooking.
pub mod exported {
    #![allow(clippy::missing_safety_doc)]

    use super::*;

    #[export_name = "Memory_Init"]
    pub unsafe extern "C" fn my_Memory_Init(buf: *mut c_void, size: c_int) -> c_int {
        abort_on_panic(move || {
            let marker = MainThreadMarker::new();

            // This is the first function called on Linux, so do due initialization.
            ensure_logging_hooks();
            ensure_profiling();
            vulkan::init();

            #[cfg(unix)]
            find_pointers(marker);

            // hw depends on SDL so it must be loaded by now.
            sdl::find_pointers(marker);

            let rv = Memory_Init.get(marker)(buf, size);

            cvars::register_all_cvars(marker);
            commands::register_all_commands(marker);
            cvars::deregister_disabled_module_cvars(marker);
            commands::deregister_disabled_module_commands(marker);

            rv
        })
    }

    #[export_name = "Host_Shutdown"]
    pub unsafe extern "C" fn my_Host_Shutdown() {
        abort_on_panic(move || {
            let marker = MainThreadMarker::new();

            commands::deregister_all_commands(marker);

            Host_Shutdown.get(marker)();

            cvars::mark_all_cvars_as_not_registered(marker);

            sdl::reset_pointers(marker);
            reset_pointers(marker);
        })
    }

    #[export_name = "V_ApplyShake"]
    pub unsafe extern "C" fn my_V_ApplyShake(
        origin: *mut c_float,
        angles: *mut c_float,
        factor: c_float,
    ) {
        abort_on_panic(move || {
            let marker = MainThreadMarker::new();

            if shake_remove::is_active(marker) {
                return;
            } else {
                V_ApplyShake.get(marker)(origin, angles, factor);
            }
        })
    }

    #[cfg_attr(not(feature = "bxt-compatibility"), export_name = "V_FadeAlpha")]
    pub unsafe extern "C" fn my_V_FadeAlpha() -> c_int {
        abort_on_panic(move || {
            let marker = MainThreadMarker::new();

            if fade_remove::is_active(marker) {
                0
            } else {
                V_FadeAlpha.get(marker)()
            }
        })
    }

    #[cfg_attr(not(feature = "bxt-compatibility"), export_name = "SV_Frame")]
    pub unsafe extern "C" fn my_SV_Frame() {
        abort_on_panic(move || {
            let marker = MainThreadMarker::new();

            tas_logging::begin_physics_frame(marker);

            SV_Frame.get(marker)();

            tas_logging::end_physics_frame(marker);
        })
    }

    #[export_name = "_Z18Sys_VID_FlipScreenv"]
    pub unsafe extern "C" fn my_Sys_VID_FlipScreen() {
        abort_on_panic(move || {
            let marker = MainThreadMarker::new();

            capture::capture_frame(marker);

            Sys_VID_FlipScreen.get(marker)();
        })
    }

    #[export_name = "ReleaseEntityDlls"]
    pub unsafe extern "C" fn my_ReleaseEntityDlls() {
        abort_on_panic(move || {
            let marker = MainThreadMarker::new();

            server::reset_entity_interface(marker);

            // After updating pointers some modules might have got disabled.
            cvars::deregister_disabled_module_cvars(marker);
            commands::deregister_disabled_module_commands(marker);

            ReleaseEntityDlls.get(marker)();
        })
    }

    #[export_name = "LoadEntityDLLs"]
    pub unsafe extern "C" fn my_LoadEntityDLLs(base_dir: *const c_char) {
        abort_on_panic(move || {
            let marker = MainThreadMarker::new();

            LoadEntityDLLs.get(marker)(base_dir);

            server::hook_entity_interface(marker);

            // After updating pointers some modules might have got disabled.
            cvars::deregister_disabled_module_cvars(marker);
            commands::deregister_disabled_module_commands(marker);
        })
    }

    #[export_name = "S_PaintChannels"]
    pub unsafe extern "C" fn my_S_PaintChannels(end_time: c_int) {
        abort_on_panic(move || {
            let marker = MainThreadMarker::new();

            if capture::skip_paint_channels(marker) {
                return;
            }

            S_PaintChannels.get(marker)(end_time);
        })
    }

    #[export_name = "S_TransferStereo16"]
    pub unsafe extern "C" fn my_S_TransferStereo16(end: c_int) {
        abort_on_panic(move || {
            let marker = MainThreadMarker::new();

            capture::on_s_transfer_stereo_16(marker, end);

            S_TransferStereo16.get(marker)(end);
        })
    }

    #[export_name = "Host_FilterTime"]
    pub unsafe extern "C" fn my_Host_FilterTime(time: c_float) -> c_int {
        abort_on_panic(move || {
            let marker = MainThreadMarker::new();

            let skip = capture::on_host_filter_time(marker);

            let rv = if skip {
                1
            } else {
                Host_FilterTime.get(marker)(time)
            };

            if rv != 0 {
                capture::time_passed(marker);
            }

            rv
        })
    }

    #[export_name = "CL_Disconnect"]
    pub unsafe extern "C" fn my_CL_Disconnect() {
        abort_on_panic(move || {
            let marker = MainThreadMarker::new();

            capture::on_cl_disconnect(marker);

            CL_Disconnect.get(marker)();
        })
    }

    #[cfg_attr(not(feature = "bxt-compatibility"), export_name = "Key_Event")]
    pub unsafe extern "C" fn my_Key_Event(key: c_int, down: c_int) {
        abort_on_panic(move || {
            let marker = MainThreadMarker::new();

            capture::on_key_event_start(marker);

            Key_Event.get(marker)(key, down);

            capture::on_key_event_end(marker);
        })
    }

    #[export_name = "Con_ToggleConsole_f"]
    pub unsafe extern "C" fn my_Con_ToggleConsole_f() {
        abort_on_panic(move || {
            let marker = MainThreadMarker::new();

            if !capture::prevent_toggle_console(marker) {
                Con_ToggleConsole_f.get(marker)();
            }
        })
    }

    #[export_name = "Host_NextDemo"]
    pub unsafe extern "C" fn my_Host_NextDemo() {
        abort_on_panic(move || {
            let marker = MainThreadMarker::new();

            Host_NextDemo.get(marker)();

            demo_playback::set_next_demo(marker);
        })
    }

    #[export_name = "R_SetFrustum"]
    pub unsafe extern "C" fn my_R_SetFrustum() {
        abort_on_panic(move || {
            let marker = MainThreadMarker::new();

            if let Some(fov) = force_fov::fov(marker) {
                *scr_fov_value.get(marker) = fov;
            }

            R_SetFrustum.get(marker)();
        })
    }
}
