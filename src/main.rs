use windows::core::Result;

use windows::Win32::UI::Input::KeyboardAndMouse::{GetAsyncKeyState, VK_CONTROL, VK_F4, VK_MENU};
use windows::Win32::UI::WindowsAndMessaging::{
    GetForegroundWindow, GetWindowTextLengthW, GetWindowThreadProcessId,
};

use windows::Win32::Foundation::{CloseHandle, HANDLE, HWND};
use windows::Win32::System::Threading::{OpenProcess, Sleep, TerminateProcess, PROCESS_TERMINATE};

fn find_top_window() -> Option<HWND> {
    let window = unsafe { GetForegroundWindow() };

    if window.is_invalid() {
        return None;
    }

    let length = unsafe { GetWindowTextLengthW(window) } as usize;

    if length == 0 {
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
    let combination = [VK_CONTROL.0, VK_MENU.0, VK_F4.0];

    combination
        .iter()
        .all(|&vk| (unsafe { GetAsyncKeyState(vk as i32) } as i16) & i16::MIN != 0)
}

fn main() -> Result<()> {
    loop {
        unsafe { Sleep(5) };

        let requested = user_requested_kill();

        if !requested {
            continue;
        }

        let top = find_top_window();

        if top.is_none() {
            continue;
        }

        let targ = handle_from_hwnd(top.unwrap());

        if targ.is_none() {
            continue;
        }

        kill_process(targ.unwrap())?;

        unsafe { Sleep(1500) };
    }
}
