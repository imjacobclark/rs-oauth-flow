use oauth2::TokenResponse;

mod oauth_integration;
mod values;

pub fn get_auth_url() -> String {
    oauth_integration::flow::get_auth_url().to_string()
}

pub async fn exchange_code_for_access_toke(code: String) -> values::io::Result<String> {
    let token_response = oauth_integration::flow::exchange_code(code)
        .await
        .map_err(|e| Box::<dyn std::error::Error>::from(format!("Failed to exchange code: {}", e)));

    token_response.and_then(|tr| {
        Ok(tr.access_token()
            .secret()
            .clone()
            .to_string())
    })
}
