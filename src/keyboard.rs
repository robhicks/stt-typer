use evdev::{Device, EventSummary, KeyCode};
use std::time::{Duration, Instant};

/// Scan `/dev/input/event*` for all keyboard devices that support KEY_RIGHTCTRL.
/// Returns an empty vec with a warning on stderr if none are found
/// (e.g. user is not in the `input` group).
pub fn find_keyboard_devices() -> Vec<Device> {
    let devices: Vec<Device> = evdev::enumerate()
        .filter_map(|(_path, device)| {
            let dominated = device.supported_keys()?.contains(KeyCode::KEY_RIGHTCTRL);
            dominated.then_some(device)
        })
        .collect();

    if devices.is_empty() {
        eprintln!(
            "stt-typer: could not find a keyboard device with KEY_RIGHTCTRL. \
             Ensure you are in the 'input' group (sudo usermod -aG input $USER, then re-login)."
        );
    }

    devices
}

/// Wait for a right CTRL key press on any of the given devices.
/// Returns `true` if the key was pressed, `false` if the timeout expired.
pub fn wait_for_right_ctrl(devices: &mut [Device], timeout: Duration) -> Result<bool, String> {
    for device in devices.iter_mut() {
        device
            .set_nonblocking(true)
            .map_err(|e| format!("failed to set device non-blocking: {e}"))?;
    }

    let start = Instant::now();

    loop {
        if start.elapsed() > timeout {
            return Ok(false);
        }

        for device in devices.iter_mut() {
            match device.fetch_events() {
                Ok(events) => {
                    for event in events {
                        if let EventSummary::Key(_, KeyCode::KEY_RIGHTCTRL, 1) =
                            event.destructure()
                        {
                            return Ok(true);
                        }
                    }
                }
                Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {}
                Err(e) => return Err(format!("failed to read input events: {e}")),
            }
        }

        std::thread::sleep(Duration::from_millis(10));
    }
}

/// Wait for the right CTRL key to be released on any of the given devices.
/// Returns `true` if the key was released, `false` if the timeout expired.
pub fn wait_for_right_ctrl_release(
    devices: &mut [Device],
    timeout: Duration,
) -> Result<bool, String> {
    for device in devices.iter_mut() {
        device
            .set_nonblocking(true)
            .map_err(|e| format!("failed to set device non-blocking: {e}"))?;
    }

    let start = Instant::now();

    loop {
        if start.elapsed() > timeout {
            return Ok(false);
        }

        for device in devices.iter_mut() {
            match device.fetch_events() {
                Ok(events) => {
                    for event in events {
                        // value 0 = key release
                        if let EventSummary::Key(_, KeyCode::KEY_RIGHTCTRL, 0) =
                            event.destructure()
                        {
                            return Ok(true);
                        }
                    }
                }
                Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {}
                Err(e) => return Err(format!("failed to read input events: {e}")),
            }
        }

        std::thread::sleep(Duration::from_millis(10));
    }
}
