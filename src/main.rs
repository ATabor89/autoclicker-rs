#[cfg(windows)] extern crate winapi;
use std::io::Error;

static SLEEP_BETWEEN_CLICKS: std::time::Duration = std::time::Duration::from_millis(50);
static SLEEP_WHEN_WAITING: std::time::Duration = std::time::Duration::from_millis(50);

#[cfg(windows)]
fn left_click() -> Result<(), Error> {
    use winapi::um::winuser::{INPUT, INPUT_u, SendInput, MOUSEEVENTF_LEFTDOWN, MOUSEEVENTF_LEFTUP, INPUT_MOUSE};

    let mut input = INPUT {
        type_: INPUT_MOUSE,
        u: INPUT_u::default(),
    };
    let mut mouse_input = unsafe { input.u.mi_mut() };
    mouse_input.dwFlags = MOUSEEVENTF_LEFTDOWN;
    unsafe {
        SendInput(1, &mut input, std::mem::size_of::<INPUT>() as i32);
    }

    let mut mouse_input = unsafe { input.u.mi_mut() };
    mouse_input.dwFlags = MOUSEEVENTF_LEFTUP;
    unsafe {
        SendInput(1, &mut input, std::mem::size_of::<INPUT>() as i32);
    }

    Ok(())
}

#[cfg(windows)]
fn key_listener() -> Result<(), Error> {
    use winapi::um::winuser::{GetAsyncKeyState, VK_F6, VK_F7};

    let mut click = false;

    loop {
        if unsafe {(GetAsyncKeyState(VK_F6) & 1) != 0} {
            click = !click;
        }

        if unsafe {(GetAsyncKeyState(VK_F7) & 1) != 0} {
            break;
        }

        if click {
            left_click().unwrap();
            std::thread::sleep(SLEEP_BETWEEN_CLICKS);
        }
        else {
            std::thread::sleep(SLEEP_WHEN_WAITING);
        }
    }

    Ok(())
}

fn main() {
    key_listener().unwrap();
}