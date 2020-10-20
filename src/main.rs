#[cfg(windows)] extern crate winapi;
use std::io::Error;
use std::collections::HashMap;

use config::*;
use notify::{DebouncedEvent, RecommendedWatcher, RecursiveMode, Watcher};
use serde::{Deserialize, Serialize};
use std::sync::mpsc::channel;
use std::sync::RwLock;

#[macro_use]
extern crate lazy_static;

#[derive(Serialize, Deserialize, Debug)]
struct Settings {
    time_between_clicks_millis: u64,
    should_left_click: bool,
    should_middle_click: bool,
    should_right_click: bool,
}

lazy_static! {
    static ref SETTINGS: RwLock<Config> = RwLock::new({
        let mut settings = Config::default();
        settings.merge(File::with_name("Settings.toml")).unwrap();

        settings
    });
}

fn show() {
    println!(
        " * Settings :: \n\x1b[31m{:?}\x1b[0m",
        SETTINGS
            .read()
            .unwrap()
            .clone()
            .try_into::<HashMap<String, String>>()
            .unwrap()
    );
}

fn get_settings() -> Settings {
    let settings = SETTINGS
        .read()
        .unwrap()
        .clone()
        .try_into::<Settings>()
        .unwrap();

    settings
}

fn watch() {
    // Create a channel to receive the events.
    let (tx, rx) = channel();

    // Automatically select the best implementation for your platform.
    // You can also access each implementation directly e.g. INotifyWatcher.
    let mut watcher: RecommendedWatcher =
        Watcher::new(tx, std::time::Duration::from_secs(2)).unwrap();

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher
        .watch("./Settings.toml", RecursiveMode::NonRecursive)
        .unwrap();

    // This is a simple loop, but you may want to use more complex logic here,
    // for example to handle I/O.
    loop {
        match rx.recv() {
            Ok(DebouncedEvent::Write(_)) => {
                println!(" * Settings.toml written; refreshing configuration ...");
                SETTINGS.write().unwrap().refresh().unwrap();
                show();
            }

            Err(e) => println!("watch error: {:?}", e),

            _ => {
                // Ignore event
            }
        }
    }
}

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

    println!("Autoclicker started. Press F6 to toggle clicking and press F7 to terminate program.");
    println!("Modify Settings.toml to change configuration (can be done while program is live).");

    let mut click = false;
    let mut settings = get_settings();

    loop {
        if unsafe {(GetAsyncKeyState(VK_F6) & 1) != 0} {
            if !click {
                settings = get_settings();
            }
            click = !click;
        }

        if unsafe {(GetAsyncKeyState(VK_F7) & 1) != 0} {
            break;
        }

        if click {
            if settings.should_left_click {
                left_click().unwrap();
            }
            if settings.should_middle_click {
                middle_click().unwrap();
            }
            if settings.should_right_click {
                right_click().unwrap();
            }
            std::thread::sleep(std::time::Duration::from_millis(settings.time_between_clicks_millis));
        }
        else {
            std::thread::sleep(SLEEP_WHEN_WAITING);
        }
    }

    Ok(())
}

fn main() {
    std::thread::spawn(|| {
        watch();
    });

    key_listener().unwrap();
}