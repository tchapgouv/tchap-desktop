//! Opt-in workaround for GTK reclaiming keyboard focus from the CEF child.
//! https://github.com/chromiumembedded/cef/issues/3782#issuecomment-2351673434

use anyhow::Context;
use raw_window_handle::{HasWindowHandle, RawWindowHandle};
use x11rb::{
    protocol::xproto::{AtomEnum, ConnectionExt, PropMode},
    wrapper::ConnectionExt as _,
};

/// Call on the UI thread after the main native window has been created.
pub fn remove_take_focus(window: &impl HasWindowHandle) -> anyhow::Result<&'static str> {
    let window_id = match window.window_handle()?.as_raw() {
        RawWindowHandle::Xlib(handle) => u32::try_from(handle.window)?,
        RawWindowHandle::Xcb(handle) => handle.window.get(),
        _ => return Ok("Skipped: native window is not X11"),
    };

    // Tauri's Linux CEF window uses the process DISPLAY (including XWayland).
    // A separate connection avoids borrowing GTK's connection across FFI.
    let (connection, _) = x11rb::connect(None)?;
    let protocols = connection.intern_atom(true, b"WM_PROTOCOLS")?.reply()?.atom;
    let take_focus = connection.intern_atom(true, b"WM_TAKE_FOCUS")?.reply()?.atom;
    anyhow::ensure!(protocols != 0, "WM_PROTOCOLS atom does not exist");
    if take_focus == 0 {
        return Ok("Unchanged: WM_TAKE_FOCUS atom is absent");
    }

    let reply = connection
        .get_property(false, window_id, protocols, AtomEnum::ATOM, 0, 1024)?
        .reply()?;
    anyhow::ensure!(
        reply.type_ == u32::from(AtomEnum::ATOM) && reply.bytes_after == 0,
        "WM_PROTOCOLS is missing, malformed, or too large; leaving it unchanged"
    );
    let mut remaining: Vec<u32> = reply
        .value32()
        .context("WM_PROTOCOLS must contain 32-bit atoms")?
        .collect();
    let original_len = remaining.len();
    remaining.retain(|atom| *atom != take_focus);
    if remaining.len() == original_len {
        return Ok("Unchanged: WM_TAKE_FOCUS is already absent");
    }

    // Preserve every other protocol, including close/ping/synchronization.
    // check() flushes the request and waits for any X11 error before returning.
    connection
        .change_property32(
            PropMode::REPLACE,
            window_id,
            protocols,
            AtomEnum::ATOM,
            &remaining,
        )?
        .check()?;
    Ok("Removed WM_TAKE_FOCUS from the main window")
}
