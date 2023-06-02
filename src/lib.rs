// Imports
use windows::{
    w,
    Win32::Foundation::*,
    Win32::{
        System::{
            LibraryLoader::*,
            SystemServices::*,
            Threading::{CreateThread, THREAD_CREATION_FLAGS},
        },
        UI::WindowsAndMessaging::*,
    },
};

#[no_mangle]
pub extern "system" fn DllMain(
    hinst_dll: HMODULE,
    fdw_reason: u32,
    _lpv_reserved: *mut std::ffi::c_void,
) -> BOOL {
    unsafe {
        OUR_HINST = hinst_dll;
    }

    unsafe {
        // DLL_THREAD_... notifications are not needed for this simple example
        if !DisableThreadLibraryCalls(hinst_dll).as_bool() {
            return false.into();
        }
    }

    match fdw_reason {
        DLL_PROCESS_ATTACH => {
            // Show our little demo message box
            unsafe {
                let thread_start_address = std::mem::transmute::<
                    extern "system" fn() -> u32,
                    unsafe extern "system" fn(*mut std::ffi::c_void) -> u32,
                >(show_message_box);
                match CreateThread(
                    None,
                    0,
                    Some(thread_start_address),
                    None,
                    THREAD_CREATION_FLAGS(0),
                    None,
                ) {
                    Ok(_) => (),
                    Err(_) => return false.into(),
                }
            }

            return true.into();
        }
        DLL_PROCESS_DETACH => {
            // We need not do anything special here as all of our allocations are on the stack
            TRUE
        }
        _ => FALSE,
    };

    FALSE
}

static mut OUR_HINST: HMODULE = HMODULE(INVALID_HANDLE_VALUE.0);

extern "system" fn show_message_box() -> u32 {
    unsafe {
        MessageBoxW(
            None,
            w!("Hello, World!"),
            w!("A message from example-dll-rs"),
            MESSAGEBOX_STYLE(0u32),
        );
    }

    unsafe {
        FreeLibraryAndExitThread(OUR_HINST, 0);
    }
}
