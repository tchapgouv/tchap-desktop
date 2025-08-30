use tauri::{App, AppHandle, Manager, Runtime};


fn toggle_activation<R: Runtime>(app: &AppHandle<R>) {
    if let Some(webview_window) = app.get_webview_window("main") {
        if let Ok(is_visible) = webview_window.is_visible() {
            if is_visible {
                let _ = webview_window.hide();
            } else {
                let _ = webview_window.unminimize();
                let _ = webview_window.show();
                let _ = webview_window.set_focus();
                let _ = webview_window.set_always_on_top(true);
            }
        }
    }
}

#[cfg(all(desktop, not(target_os = "linux")))]
use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};

#[cfg(all(desktop, not(target_os = "linux")))]
pub fn tray(app: &mut App) {
    TrayIconBuilder::new()
        .icon(app.default_window_icon().unwrap().clone())
        //focus on main window when clicking the tray icon
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                toggle_activation(tray.app_handle());
            }
        })
        .build(app).unwrap();
}


#[cfg(target_os = "linux")]
use ksni::blocking::TrayMethods;

#[cfg(target_os = "linux")]
#[derive(Debug)]
struct TchapTray<R: Runtime> {
    app_handle: AppHandle<R>,
}

#[cfg(target_os = "linux")]
impl<R: Runtime> TchapTray<R> {
    fn new<M: Manager<R>>(manager: &M) -> Self {
        Self { app_handle: manager.app_handle().clone() }
    }
}

#[cfg(target_os = "linux")]
impl<R: Runtime> ksni::Tray for TchapTray<R> {
    fn id(&self) -> String {
        String::from("Tchap")
    }

    fn activate(&mut self, _x: i32, _y: i32) {
        toggle_activation(&self.app_handle)
    }

    fn icon_pixmap(&self) -> Vec<ksni::Icon> {
        let icon = self.app_handle.default_window_icon().unwrap();
        let rgba = icon.rgba().to_vec();

        // convert from rgba to argb
        let mut bytes = rgba;
        for i in 0..(bytes.len() / 4) {
            let j = i * 4;
            let a = bytes[j + 3];
            bytes[j + 3] = bytes[j + 2];
            bytes[j + 2] = bytes[j + 1];
            bytes[j + 1] = bytes[j];
            bytes[j] = a;
        }

        vec![
            ksni::Icon {
                height: icon.height().cast_signed(),
                width: icon.width().cast_signed(),
                data: bytes,
            }
        ]
    }
}

#[cfg(target_os = "linux")]
pub fn tray(app: &mut App) {
    let tray = TchapTray::new(app.app_handle());
    tray.spawn().unwrap();
}