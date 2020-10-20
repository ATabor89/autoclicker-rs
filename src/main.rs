#[cfg(windows)] extern crate winapi;
use std::io::Error;

// EDIT TIME BETWEEN CLICKS HERE
static SLEEP_BETWEEN_CLICKS: std::time::Duration = std::time::Duration::from_millis(50);

// LEAVE THIS ONE ALONE
// Sleep keeps the loop from killing too much CPU time when idle
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
fn right_click() -> Result<(), Error> {
    use winapi::um::winuser::{INPUT, INPUT_u, SendInput, MOUSEEVENTF_RIGHTDOWN, MOUSEEVENTF_RIGHTUP, INPUT_MOUSE};

    let mut input = INPUT {
        type_: INPUT_MOUSE,
        u: INPUT_u::default(),
    };
    let mut mouse_input = unsafe { input.u.mi_mut() };
    mouse_input.dwFlags = MOUSEEVENTF_RIGHTDOWN;
    unsafe {
        SendInput(1, &mut input, std::mem::size_of::<INPUT>() as i32);
    }

    let mut mouse_input = unsafe { input.u.mi_mut() };
    mouse_input.dwFlags = MOUSEEVENTF_RIGHTUP;
    unsafe {
        SendInput(1, &mut input, std::mem::size_of::<INPUT>() as i32);
    }

    Ok(())
}

#[cfg(windows)]
fn middle_click() -> Result<(), Error> {
    use winapi::um::winuser::{INPUT, INPUT_u, SendInput, MOUSEEVENTF_MIDDLEDOWN, MOUSEEVENTF_MIDDLEUP, INPUT_MOUSE};

    let mut input = INPUT {
        type_: INPUT_MOUSE,
        u: INPUT_u::default(),
    };
    let mut mouse_input = unsafe { input.u.mi_mut() };
    mouse_input.dwFlags = MOUSEEVENTF_MIDDLEDOWN;
    unsafe {
        SendInput(1, &mut input, std::mem::size_of::<INPUT>() as i32);
    }

    let mut mouse_input = unsafe { input.u.mi_mut() };
    mouse_input.dwFlags = MOUSEEVENTF_MIDDLEUP;
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
            right_click().unwrap();
            middle_click().unwrap();
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