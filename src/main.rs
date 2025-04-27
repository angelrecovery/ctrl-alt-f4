use windows::Win32::{
    Foundation::{CloseHandle, HANDLE, HWND},
    System::Threading::{OpenProcess, PROCESS_ACCESS_RIGHTS, PROCESS_TERMINATE, TerminateProcess},
    UI::{
        Input::KeyboardAndMouse::{GetAsyncKeyState, VK_CONTROL, VK_F4, VK_MENU},
        WindowsAndMessaging::{GetForegroundWindow, GetWindowThreadProcessId},
    },
};
use windows::core::Result;

fn top_hwnd() -> Option<HWND> {
    let window = unsafe { GetForegroundWindow() };
    if window.is_invalid() {
        None
    } else {
        Some(window)
    }
}

fn handle_from_hwnd(window: HWND, access_type: PROCESS_ACCESS_RIGHTS) -> Option<HANDLE> {
    let mut pid = 0;
    unsafe {
        if GetWindowThreadProcessId(window, Some(&mut pid)) == 0 {
            return None;
        }

        OpenProcess(access_type, false, pid).ok()
    }
}

fn kill_process(handle: HANDLE) -> Result<()> {
    unsafe {
        TerminateProcess(handle, 0)?;
        CloseHandle(handle)
    }
}

fn req_kill() -> bool {
    [VK_CONTROL.0, VK_MENU.0, VK_F4.0]
        .iter()
        .all(|&vk| (unsafe { GetAsyncKeyState(vk.into()) }) & i16::MIN != 0)
}

fn main() -> Result<()> {
    println!("(ctrl-alt-f4) Started");
    loop {
        std::thread::sleep(std::time::Duration::from_millis(5));

        if !req_kill() {
            continue;
        }

        let Some(hwnd) = top_hwnd() else {
            continue;
        };

        let Some(handle) = handle_from_hwnd(hwnd, PROCESS_TERMINATE) else {
            continue;
        };

        if kill_process(handle).is_err() {
            eprintln!(
                "(ctrl-alt-f4) Failed to kill process: {}",
                std::io::Error::last_os_error()
            );
        }

        std::thread::sleep(std::time::Duration::from_millis(1500));
    }
}
