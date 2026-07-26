use std::{path::Path, process::Command};

pub fn launch_qq() -> anyhow::Result<()>
{
    let status = Command::new("open")
        .args(["-a", "QQ"])
        .status()
        .context("failed to execute `open`")?;

    if !status.success() {
        anyhow::bail!("failed to launch QQ");
    }

    Ok(())
}

pub fn activate_qq_window() -> anyhow::Result<()>
{
    let status = Command::new("open")
        .args(["-a", "QQ"])
        .status()
        .context("failed to execute `open`")?;

    if !status.success() {
        anyhow::bail!("failed to activate QQ");
    }

    Ok(())
}

use anyhow::bail;
use objc2_app_kit::NSWorkspace;
use std::{
    thread::sleep,
    time::{Duration, Instant},
};

pub fn wait_for_qq_foreground(timeout: std::time::Duration) -> anyhow::Result<()>
{
    let deadline = Instant::now() + timeout;

    while Instant::now() < deadline {
        let workspace = NSWorkspace::sharedWorkspace();

        if let Some(app) = workspace.frontmostApplication() {
            if let Some(bundle_id) = app.bundleIdentifier() {
                if bundle_id.to_string() == "com.tencent.qq" {
                    return Ok(());
                }
            }
        }

        sleep(Duration::from_millis(10));
    }

    bail!("Timed out waiting for QQ to become foreground");
}

use core_graphics::event::{CGEvent, CGEventFlags, CGEventTapLocation, CGKeyCode};

pub fn search_contact(qq: &str) -> anyhow::Result<()>
{
    // ⌘ + F
    key_down(3, true)?; // F
    key_up(3, true)?;

    sleep(Duration::from_millis(80));

    // 输入 QQ 号
    type_text(qq)?;

    Ok(())
}

fn key_down(key: CGKeyCode, command: bool) -> Result<()>
{
    let mut event = CGEvent::new_keyboard_event(None, key, true).unwrap();

    if command {
        event.set_flags(CGEventFlags::CGEventFlagMaskCommand);
    }

    event.post(CGEventTapLocation::HID);

    Ok(())
}

fn key_up(key: CGKeyCode, command: bool) -> Result<()>
{
    let mut event = CGEvent::new_keyboard_event(None, key, false).unwrap();

    if command {
        event.set_flags(CGEventFlags::CGEventFlagMaskCommand);
    }

    event.post(CGEventTapLocation::HID);

    Ok(())
}

fn type_text(text: &str) -> anyhow::Result<()>
{
    for ch in text.encode_utf16() {
        let down = CGEvent::new_keyboard_event(None, 0, true).unwrap();
        down.set_string_from_utf16(&[ch]);
        down.post(CGEventTapLocation::HID);

        let up = CGEvent::new_keyboard_event(None, 0, false).unwrap();
        up.set_string_from_utf16(&[ch]);
        up.post(CGEventTapLocation::HID);
    }

    Ok(())
}

pub fn wait_for_search_result(timeout: std::time::Duration) -> anyhow::Result<()> {}

pub fn open_first_search_result() -> anyhow::Result<()> {}

pub fn wait_for_chat_ready(timeout: std::time::Duration) -> anyhow::Result<()> {}

pub fn focus_chat_input() -> anyhow::Result<()> {}

pub fn replace_chat_input(text: &str) -> anyhow::Result<()> {}

pub fn send_message(text: &str, to: &str) -> anyhow::Result<()> {}
