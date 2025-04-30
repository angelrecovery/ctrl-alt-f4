use windows::Win32::{
    Foundation::{CloseHandle, HANDLE, HWND},
    System::Threading::{OpenProcess, PROCESS_ACCESS_RIGHTS, PROCESS_TERMINATE, TerminateProcess},
    UI::{
        Input::KeyboardAndMouse::{GetAsyncKeyState, VK_CONTROL, VK_F4, VK_MENU},
        WindowsAndMessaging::{GetForegroundWindow, GetWindowThreadProcessId},
    },
};
use windows::core::Result;

struct Handle(HANDLE);

impl Handle {
    pub fn new(handle: HANDLE) -> Option<Self> {
        if handle.is_invalid() {
            None
        } else {
            Some(Self(handle))
        }
    }

    pub fn raw(self) -> HANDLE {
        self.0
    }
}

impl Drop for Handle {
    fn drop(&mut self) {
        unsafe { _ = CloseHandle(self.0) }
    }
}

fn top_window() -> Option<HWND> {
    let window = unsafe { GetForegroundWindow() };
    if window.is_invalid() {
        None
    } else {
        Some(window)
    }
}

fn pid_from_window(window: HWND) -> Option<u32> {
    let mut pid = 0;
    if unsafe { GetWindowThreadProcessId(window, Some(&mut pid)) } == 0 {
        None
    } else {
        Some(pid)
    }
}

fn handle_from_pid(pid: u32, rights: PROCESS_ACCESS_RIGHTS) -> Option<Handle> {
    if pid == std::process::id() {
        return None;
    }
    if let Some(handle) = unsafe { OpenProcess(rights, false, pid).ok() } {
        Handle::new(handle)
    } else {
        None
    }
}

fn req_kill() -> bool {
    [VK_CONTROL.0, VK_MENU.0, VK_F4.0]
        .iter()
        .all(|&vk| (unsafe { GetAsyncKeyState(vk.into()) }) & i16::MIN != 0)
}

fn kill(handle: Handle) -> Result<()> {
     unsafe { TerminateProcess(handle.raw(), 0) }
}

fn main() -> Result<()> {
    println!("(ctrl-alt-f4) Started");
    loop {
        std::thread::sleep(std::time::Duration::from_millis(5));

        if !req_kill() {
            continue;
        }

        let Some(window) = top_window() else {
            continue;
        };

        let Some(pid) = pid_from_window(window) else {
            continue;
        };

        let Some(handle) = handle_from_pid(pid, PROCESS_TERMINATE) else {
            continue;
        };

        if let Err(error) = kill(handle) {
            eprintln!(
                "(ctrl-alt-f4) Failed to kill process: {}, {}",
                std::io::Error::last_os_error(),
                error
            );
        }

        std::thread::sleep(std::time::Duration::from_millis(1500));
    }
}
