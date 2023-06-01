// Imports
use windows::{
    w,
    Win32::Foundation::*,
    Win32::{
        System::{LibraryLoader::*, SystemServices::*},
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
        // DLL_THREAD_... notifications are not needed for this simple example
        if !DisableThreadLibraryCalls(hinst_dll).as_bool() {
            return false.into();
        }
    }

    match fdw_reason {
        DLL_PROCESS_ATTACH => {
            // Show our little demo message box
            show_message_box();

            unsafe {
                FreeLibraryAndExitThread(hinst_dll, 0);
            }

            // We won't ever reach this
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

fn show_message_box() {
    unsafe {
        MessageBoxW(
            None,
            w!("Hello, World!"),
            w!("A message from example-dll-rs"),
            MESSAGEBOX_STYLE(0u32),
        );
    }
}
