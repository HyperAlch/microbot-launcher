use base64::{engine::general_purpose, Engine as _};
use form_urlencoded;
use reqwest::header::{self, ACCEPT, CONTENT_TYPE};
use serde::Deserialize;
use std::{collections::HashMap, format};
use std::{println, str};

pub async fn generate_login_url() -> String {
    let (verifier, challenge) = generate_pkce_pair();

    let url = format!("{}/oauth2/auth?", Config::OriginUrl.value());

    let query_string: String = form_urlencoded::Serializer::new(String::new())
        .append_pair("auth_method", "")
        .append_pair("login_type", "")
        .append_pair("flow", "launcher")
        .append_pair("response_type", "code")
        .append_pair("client_id", &Config::ClientId.value())
        .append_pair("redirect_uri", &Config::RedirectUrl.value())
        .append_pair("code_challenge", &challenge)
        .append_pair("code_challenge_method", "S256")
        .append_pair("prompt", "login")
        .append_pair(
            "scope",
            "openid offline gamesso.token.create user.profile.read",
        )
        .append_pair(
            "state",
            str::from_utf8(&verifier).expect("Error in: generate_login_url() - verifier invalid"),
        )
        .finish();

    format!("{}{}", url, query_string)
}

fn generate_pkce_pair() -> (Vec<u8>, String) {
    let code_verify = pkce::code_verifier(43);
    let code_challenge = pkce::code_challenge(&code_verify);

    (code_verify, code_challenge)
}

pub async fn get_login_data(params: String, user_agent: String) {
    let param_dict = get_login_precursors(params);

    let code = param_dict.get("code").expect("Failed to parse code");
    let state = param_dict.get("state").expect("Failed to parse state");

    println!("Code: {:?}", code);
    println!("State: {:?}", state);
    println!("User Agent: {:?}", user_agent);

    let mut headers = header::HeaderMap::new();

    // Client headers
    headers.insert(
        CONTENT_TYPE,
        "application/x-www-form-urlencoded"
            .parse()
            .expect("Invalid `Content-Type` header"),
    );
    headers.insert(
        ACCEPT,
        "application/json".parse().expect("Invalid `Accept` header"),
    );

    let client_id = Config::ClientId.value();
    let redirect_uri = Config::RedirectUrl.value();

    let mut params = HashMap::new();
    params.insert("grant_type", "authorization_code");
    params.insert("client_id", &client_id);
    params.insert("code", code);
    params.insert("code_verifier", &state);
    params.insert("redirect_uri", &redirect_uri);

    // Client builder
    let client = reqwest::Client::builder()
        .default_headers(headers)
        .user_agent(user_agent)
        .build()
        .expect("Failed to build `reqwest` http client");

    let response = client
        .post(Config::ExchangeUrl.value())
        .form(&params)
        .send()
        .await
        .expect(&format!(
            "Failed POST request: {}",
            Config::ExchangeUrl.value()
        ));

    let login_data = response.json::<LoginData>().await.unwrap();
    println!("\n\n[Login Data]\n{:?}", login_data);
}

fn get_login_precursors(mut params: String) -> HashMap<String, String> {
    params = params.replace("jagex:", "");
    let params: Vec<(String, String)> = params
        .split(",")
        .map(|x| x.to_string())
        .map(|x| {
            let split: Vec<&str> = x.split("=").collect();
            (
                split
                    .get(0)
                    .expect(&format!("Failed parsing code, state, or intent: {}", x))
                    .to_string(),
                split
                    .get(1)
                    .expect(&format!("Failed parsing code, state, or intent: {}", x))
                    .to_string(),
            )
        })
        .collect();

    let mut param_dict: HashMap<String, String> = Default::default();
    for param in params {
        param_dict.insert(param.0, param.1);
    }

    param_dict
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct AddingAccountPayload {
    pub url: String,
    pub user_agent: String,
}

#[derive(Deserialize, Debug)]
struct LoginData {
    access_token: String,
    expires_in: u16,
    id_token: String,
    refresh_token: String,
    scope: String,
    token_type: String,
}

pub enum Config {
    Provider,
    OriginUrl,
    Origin2fa,
    RedirectUrl,
    ExchangeUrl,
    RevokeUrl,
    ClientId,
    Api,
    AuthApi,
    ProfileApi,
    ShieldUrl,
    ContentUrl,
    DefaultConfigUri,
    BasicAuthHeader,
}

impl Config {
    pub fn value(&self) -> String {
        let mut basic_auth_header = String::new();
        general_purpose::STANDARD
            .encode_string("com_jagex_auth_desktop_osrs:public", &mut basic_auth_header);

        match *self {
            Self::Provider => "runescape".to_string(),
            Self::OriginUrl => "https://account.jagex.com".to_string(),
            Self::Origin2fa => "https://secure.runescape.com".to_string(),
            Self::RedirectUrl => {
                "https://secure.runescape.com/m=weblogin/launcher-redirect".to_string()
            }
            Self::ExchangeUrl => "https://account.jagex.com/oauth2/token".to_string(),
            Self::RevokeUrl => "https://account.jagex.com/oauth2/revoke".to_string(),
            Self::ClientId => "com_jagex_auth_desktop_launcher".to_string(),
            Self::Api => "https://api.jagex.com/v1".to_string(),
            Self::AuthApi => "https://auth.jagex.com/game-session/v1".to_string(),
            Self::ProfileApi => "https://secure.jagex.com/rs-profile/v1".to_string(),
            Self::ShieldUrl => "https://auth.jagex.com/shield/oauth/token".to_string(),
            Self::ContentUrl => "https://auth.jagex.com/shield/oauth/token".to_string(),
            Self::DefaultConfigUri => "https://www.runescape.com/k=5/l=0/jav_config.ws".to_string(),
            Self::BasicAuthHeader => basic_auth_header,
        }
    }
}
