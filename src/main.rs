use windows::core::Result;

use windows::Win32::UI::Input::KeyboardAndMouse::{GetAsyncKeyState, VK_CONTROL, VK_F4, VK_MENU};
use windows::Win32::UI::WindowsAndMessaging::{GetForegroundWindow, GetWindowThreadProcessId};

use windows::Win32::Foundation::{CloseHandle, HANDLE, HWND};
use windows::Win32::System::Threading::{OpenProcess, Sleep, TerminateProcess, PROCESS_TERMINATE};

fn find_top_window() -> Option<HWND> {
    let window = unsafe { GetForegroundWindow() };

    if window.is_invalid() {
        return None;
    }

    Some(window)
}

fn handle_from_hwnd(window: HWND) -> Option<HANDLE> {
    let mut pid = 0;

    if unsafe { GetWindowThreadProcessId(window, Some(&mut pid)) } == 0 {
        return None;
    }

    unsafe { OpenProcess(PROCESS_TERMINATE, false, pid) }.ok()
}

fn kill_process(handle: HANDLE) -> Result<()> {
    unsafe { TerminateProcess(handle, 0) }?;
    unsafe { CloseHandle(handle) }
}

fn user_requested_kill() -> bool {
    const KEY_COMBO: [u16; 3] = [VK_CONTROL.0, VK_MENU.0, VK_F4.0];

    KEY_COMBO
        .iter()
        .all(|&vk| (unsafe { GetAsyncKeyState(vk.into()) }) & i16::MIN != 0)
}

fn main() -> Result<()> {
    loop {
        unsafe { Sleep(5) };

        if !user_requested_kill() {
            continue;
        }

        let Some(window) = find_top_window() else {
            continue;
        };

        let Some(handle) = handle_from_hwnd(window) else {
            continue;
        };

        kill_process(handle)?;

        unsafe { Sleep(1500) };
    }
}
