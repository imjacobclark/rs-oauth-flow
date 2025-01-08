use super::client::create_oauth_client;

use oauth2::{
    basic::BasicClient as BaseClient, CsrfToken
};
use reqwest::Url;

type ExchangeCode = Result<
    oauth2::StandardTokenResponse<oauth2::EmptyExtraTokenFields, oauth2::basic::BasicTokenType>,
    oauth2::RequestTokenError<
        oauth2::reqwest::AsyncHttpClientError, oauth2::StandardErrorResponse<oauth2::basic::BasicErrorResponseType>
    >
>;

fn get_client() -> BaseClient {
    match create_oauth_client() {
        Ok(client) => client,
        Err(e) => panic!("Failed to create OAuth client: {}", e)
    }
}

pub fn get_auth_url() -> Url {
    let (auth_url, _csrf): (Url, CsrfToken)= get_client()
        .authorize_url(CsrfToken::new_random)
        .url();

    auth_url
}

pub async fn exchange_code(code: String) -> ExchangeCode {
    get_client()
        .exchange_code(oauth2::AuthorizationCode::new(code))
        .add_extra_param("accept", "application/json")
        .add_extra_param("response_type", "token")
        .request_async(oauth2::reqwest::async_http_client)
        .await  
}