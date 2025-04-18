use windows::core::Result;

use windows::Win32::UI::Input::KeyboardAndMouse::{GetAsyncKeyState, VK_CONTROL, VK_F4, VK_MENU};
use windows::Win32::UI::WindowsAndMessaging::{GetForegroundWindow, GetWindowThreadProcessId};

use windows::Win32::Foundation::{CloseHandle, HANDLE, HWND};
use windows::Win32::System::Threading::{OpenProcess, PROCESS_TERMINATE, Sleep, TerminateProcess};

fn top_hwnd() -> Option<HWND> {
    let window = unsafe { GetForegroundWindow() };

    if window.is_invalid() {
        return None;
    }

    Some(window)
}

fn handle_from_hwnd(window: HWND) -> Option<HANDLE> {
    let mut pid = 0;
    unsafe {
        if GetWindowThreadProcessId(window, Some(&mut pid)) == 0 {
            return None;
        }
        OpenProcess(PROCESS_TERMINATE, false, pid).ok()
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
    loop {
        unsafe { Sleep(5) };

        if !req_kill() {
            continue;
        }

        let Some(hwnd) = top_hwnd() else {
            continue;
        };

        let Some(handle) = handle_from_hwnd(hwnd) else {
            continue;
        };

        if kill_process(handle).is_err() {
            eprintln!(
                "(ctrl-alt-f4) Failed to kill process: {}",
                std::io::Error::last_os_error()
            );
        }

        unsafe { Sleep(1500) };
    }
}
