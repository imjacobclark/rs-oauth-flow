pub enum OAuthConfigKeys {
    ClientId,
    ClientSecret,
    RedirectUrl,
    AuthorizeEndpoint,
    TokenEndpoint,
}

impl OAuthConfigKeys {
    pub fn as_str(&self) -> &'static str {
        match self {
            OAuthConfigKeys::ClientId => "CLIENT_ID",
            OAuthConfigKeys::ClientSecret => "CLIENT_SECRET",
            OAuthConfigKeys::RedirectUrl => "REDIRECT_URL",
            OAuthConfigKeys::AuthorizeEndpoint => "AUTHORIZE_ENDPOINT",
            OAuthConfigKeys::TokenEndpoint => "ACCESS_TOKEN_ENDPOINT",
        }
    }
}