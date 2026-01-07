use crate::common_error::CommonError;

#[tauri::command]
pub fn get_password(service: &str, user: &str) -> Result<Option<String>, CommonError> {
    let entry = keyring::Entry::new(service, user)?;
    match entry.get_password() {
        Ok(pwd) => Ok(Some(pwd)),
        Err(keyring::Error::NoEntry) => Ok(None),
        Err(e) => Err(CommonError::Keyring(e)),
    }
}

#[tauri::command]
pub fn set_password(service: &str, user: &str, password: &str) -> Result<(), CommonError> {
    keyring::Entry::new(service, user)?.set_password(password)?;
    Ok(())
}

#[tauri::command]
pub fn delete_password(service: &str, user: &str) -> Result<(), CommonError> {
    keyring::Entry::new(service, user)?.delete_credential()?;
    Ok(())
}
