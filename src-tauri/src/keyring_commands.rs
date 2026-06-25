use crate::common_error::CommonError;
use tauri::{AppHandle, Config, Runtime};

pub fn get_service_url(config: &Config) -> &'static str {
    match config.product_name.as_deref() {
        Some(name) if name.contains("dev") => "https://www.tchap.incubateur.net",
        Some(name) if name.contains("preprod") => "https://www.beta.tchap.gouv.fr",
        _ => "https://tchap.gouv.fr",
    }
}

#[tauri::command]
pub fn get_password<R: Runtime>(
    app_handle: AppHandle<R>,
    user: &str,
) -> Result<Option<String>, CommonError> {
    // service is the base url of the app depending of the ENV
    let config = app_handle.config();
    let service = get_service_url(config);
    let entry = keyring::Entry::new(service, user)?;
    match entry.get_password() {
        Ok(pwd) => Ok(Some(pwd)),
        Err(keyring::Error::NoEntry) => Ok(None),
        Err(e) => Err(CommonError::Keyring(e)),
    }
}

#[tauri::command]
pub fn set_password<R: Runtime>(
    app_handle: AppHandle<R>,
    user: &str,
    password: &str,
) -> Result<(), CommonError> {
    let config = app_handle.config();
    let service = get_service_url(config);
    keyring::Entry::new(service, user)?.set_password(password)?;
    Ok(())
}

#[tauri::command]
pub fn delete_password<R: Runtime>(
    app_handle: AppHandle<R>,
    user: &str,
) -> Result<(), CommonError> {
    let config = app_handle.config();
    let service = get_service_url(config);
    println!("delete password {:?}", service);
    keyring::Entry::new(service, user)?.delete_credential()?;
    Ok(())
}
