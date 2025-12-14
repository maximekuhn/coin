use auth_models::Session;
use axum_extra::extract::{
    CookieJar,
    cookie::{Cookie, SameSite},
};
use base64::{Engine, prelude::BASE64_STANDARD};
use time::{OffsetDateTime, UtcOffset};

const COOKIE_NAME: &str = "coin";

#[derive(Debug)]
pub enum Environment {
    Dev,
    Prod { domain: String },
}

pub fn generate_cookie(session: &Session, env: Environment) -> Cookie<'static> {
    let session_id = BASE64_STANDARD.encode(session.id);
    let mut cookie = Cookie::new(COOKIE_NAME, session_id);

    let ts = session.expires_at.timestamp();
    let odt = OffsetDateTime::from_unix_timestamp(ts)
        .expect("valid timestamp")
        .to_offset(UtcOffset::UTC);
    cookie.set_expires(odt);

    cookie.set_path("/");
    cookie.set_secure(true);
    cookie.set_same_site(SameSite::Strict);
    cookie.set_http_only(true);

    match env {
        Environment::Dev => {
            cookie.set_secure(false);
            cookie.set_domain("localhost");
        }
        Environment::Prod { domain } => {
            cookie.set_domain(domain);
        }
    }

    cookie
}

pub fn get_valid_session_id_from_cookie(jar: &CookieJar) -> Option<[u8; 128]> {
    let cookie = jar.get(COOKIE_NAME)?;
    let Ok(decoded_val) = BASE64_STANDARD.decode(cookie.value()) else {
        return None;
    };
    if decoded_val.len() != 128 {
        return None;
    }
    decoded_val.try_into().ok()
}

pub fn remove_cookie(jar: CookieJar) -> CookieJar {
    jar.remove(COOKIE_NAME)
}
