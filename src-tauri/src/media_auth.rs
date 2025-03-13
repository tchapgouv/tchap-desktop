
/// Get supported versions from the frontend
/// Get supported versions from the frontend
#[tauri::command]
pub async fn get_supported_versions(app: AppHandle) -> Result<Vec<String>, CommonError> {
    app.emit_all("serverSupportedVersions-request", ())
        .map_err(|e| CommonError::CommandError(e.to_string()))?;
    let versions = app.once::<Vec<String>>("serverSupportedVersions-response").await;
    Ok(versions.unwrap_or_default())
}

/// Get access token from the frontend 
#[tauri::command]
pub async fn get_access_token(app: AppHandle) -> Result<Option<String>, CommonError> {
    app.emit_all("userAccessToken-request", ())
        .map_err(|e| CommonError::CommandError(e.to_string()))?;
    let token = app.once::<Option<String>>("userAccessToken-response").await;
    Ok(token.unwrap_or(None))
}

/// Get homeserver URL from the frontend
#[tauri::command]
pub async fn get_homeserver_url(app: AppHandle) -> Result<Option<String>, CommonError> {
    app.emit_all("homeserverUrl-request", ())
        .map_err(|e| CommonError::CommandError(e.to_string()))?;
    let url = app.once::<Option<String>>("homeserverUrl-response").await;
    Ok(url.unwrap_or(None))
}

pub fn setup_media_auth<R: Runtime>(app_handle: AppHandle<R>) { 
    // Register before_request handler for URL rewriting
    app_handle().before_request(move |request| {
        
        Box::pin(async move {
            if let Ok(mut url) = Url::parse(request.url()) {
                let path = url.path();
                
                // Check if this is a media URL we need to handle
                if !path.starts_with("/_matrix/media/v3/download") && 
                    !path.starts_with("/_matrix/media/v3/thumbnail") {
                    return Ok(None); // Continue with unmodified request
                }
                
                // Get necessary information from frontend
                let supported_versions = get_supported_versions(&window).await;
                let access_token = get_access_token(&window).await;
                
                // Check if we need to rewrite the URL
                if supported_versions.contains(&"v1.11".to_string()) && access_token.is_some() {
                    let new_path = path.replace("/media/v3/", "/client/v1/media/");
                    url.set_path(&new_path);
                    Ok(Some(url.to_string())) // Return modified URL
                } else {
                    Ok(None) // Continue with unmodified request
                }
            } else {
                Ok(None)
            }
        }) as BoxFuture<'static, tauri::Result<Option<String>>>
    });
    
    // Register on_request handler for adding headers
    app_handle().on_request(move |request| {
        Box::pin(async move {
            if let Ok(url) = Url::parse(request.url()) {
                // Check if this is a media URL
                if !url.path().starts_with("/_matrix/client/v1/media") {
                    return Ok(None);
                }
                
                // Check if request is going to homeserver
                if let Some(homeserver) = get_homeserver_url(&window).await {
                    if let Ok(homeserver_url) = Url::parse(&homeserver) {
                        if url.origin() == homeserver_url.origin() {
                            // Add authorization header if we have a token
                            if let Some(token) = get_access_token(&window).await {
                                let mut headers = request.headers().clone();
                                headers.insert(
                                    "Authorization",
                                    format!("Bearer {}", token).parse().unwrap()
                                );
                                return Ok(Some(headers));
                            }
                        }
                    }
                }
            }
            Ok(None)
        }) as BoxFuture<'static, tauri::Result<Option<http::HeaderMap>>>
    });
}