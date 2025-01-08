use oauth2::{AuthUrl, ClientId, ClientSecret, RedirectUrl, TokenUrl};

use oauth2::basic::BasicClient as BaseClient;

use crate::values::{io, config_value::ConfigValue};
use crate::oauth_integration::config::OAuthConfigKeys;

pub struct Client {
    client_id: ClientId,
    client_secret: Option<ClientSecret>,
    auth_url: AuthUrl,
    token_url: Option<TokenUrl>,
    redirect_url: RedirectUrl,
}

fn create_oauth_value<T, E>(
    constructor: impl Fn(String) -> io::Result<T, E>,
    value_provider: impl FnOnce() -> &'static str,
) -> io::Result<T, E> 
where 
    E: std::error::Error + 'static 
{
    let env_var_name = match ConfigValue::from_env(value_provider())
        .map(|v| v.0) {
            Ok(name) => name,
            Err(e) => return constructor(format!("Failed to get value from environment: {}", e))
    };
    
    constructor(env_var_name)
}

impl Client {
    fn new() -> io::Result<Self> {
        Ok(Self {
            client_id: create_oauth_value::<ClientId, std::convert::Infallible>(
                |s| Ok(ClientId::new(s)), 
                || OAuthConfigKeys::ClientId.as_str()
            )?,
            client_secret: Some(create_oauth_value::<ClientSecret, std::convert::Infallible>(
                |s| Ok(ClientSecret::new(s)), 
                || OAuthConfigKeys::ClientSecret.as_str()
            )?),
            auth_url: create_oauth_value::<AuthUrl, oauth2::url::ParseError>(
                AuthUrl::new, 
                || OAuthConfigKeys::AuthorizeEndpoint.as_str()
            )?,
            token_url: Some(create_oauth_value::<TokenUrl, oauth2::url::ParseError>(
                TokenUrl::new, 
                || OAuthConfigKeys::TokenEndpoint.as_str()
            )?),
            redirect_url: create_oauth_value::<RedirectUrl, oauth2::url::ParseError>(
                RedirectUrl::new, 
                || OAuthConfigKeys::RedirectUrl.as_str()
            )?,
        })
    }
}

impl From<Client> for BaseClient {
    fn from(client: Client) -> Self { 
        BaseClient::new(
            client.client_id,
            client.client_secret,
            client.auth_url,
            client.token_url,
        )
        .set_redirect_uri(client.redirect_url)
    }
}

pub fn create_oauth_client() -> io::Result<BaseClient> {
    Client::new().map(BaseClient::from)
}