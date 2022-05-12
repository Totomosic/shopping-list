use std::ops::Deref;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::db::{HmacSha256, StateInstance};
use crate::models::User;
use crate::responses::{error_response, JsonResponse};

use jwt::{Error, SignWithKey, VerifyWithKey};
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Outcome, Request};

use serde_derive::{Deserialize, Serialize};

const REFRESH_TOKEN_EXPIRY: u128 = 30 * 24 * 60 * 60 * 1000;
const ACCESS_TOKEN_EXPIRY: u128 = 5 * 60 * 1000;

fn create_error(status: Status, message: &str) -> (Status, JsonResponse) {
    (status, error_response(status, message))
}

#[derive(Serialize, Deserialize)]
pub struct RefreshJwtToken {
    pub token_type: String,
    pub exp: u128,
    pub user_id: i32,
    pub display_name: String,
    pub is_admin: bool,
}

#[derive(Serialize, Deserialize)]
pub struct AccessJwtToken {
    pub token_type: String,
    pub exp: u128,
    pub user_id: i32,
    pub display_name: String,
    pub is_admin: bool,
}

pub fn get_timestamp_ms() -> u128 {
    let current = SystemTime::now();
    let since_epoch = current
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards!");
    return since_epoch.as_millis();
}

pub fn is_expired(timestamp: u128) -> bool {
    timestamp < get_timestamp_ms()
}

pub fn generate_refresh_token(user: &User, key: &HmacSha256) -> String {
    let token = RefreshJwtToken {
        token_type: String::from("refresh"),
        exp: get_timestamp_ms() + REFRESH_TOKEN_EXPIRY,
        user_id: user.id,
        display_name: user.display_name.clone(),
        is_admin: user.is_admin,
    };
    let token_string = token.sign_with_key(key);
    return token_string.unwrap_or(String::default());
}

pub fn verify_refresh_token(token: &str, key: &HmacSha256) -> Option<RefreshJwtToken> {
    let tok: Result<RefreshJwtToken, Error> = token.verify_with_key(key);
    match tok {
        Ok(result) => {
            if result.token_type == "refresh" && !is_expired(result.exp) {
                Some(result)
            } else {
                None
            }
        }
        _ => None,
    }
}

pub fn generate_access_token(user: &User, key: &HmacSha256) -> String {
    let token = AccessJwtToken {
        token_type: String::from("access"),
        exp: get_timestamp_ms() + ACCESS_TOKEN_EXPIRY,
        user_id: user.id,
        display_name: user.display_name.clone(),
        is_admin: user.is_admin,
    };
    let token_string = token.sign_with_key(key);
    return token_string.unwrap_or(String::default());
}

pub fn verify_access_token(token: &str, key: &HmacSha256) -> Option<AccessJwtToken> {
    let tok: Result<AccessJwtToken, Error> = token.verify_with_key(key);
    match tok {
        Ok(result) => {
            if result.token_type == "access" && !is_expired(result.exp) {
                Some(result)
            } else {
                None
            }
        }
        _ => None,
    }
}

fn extract_jwt_from_header(header_value: &str) -> Option<String> {
    let token = "Bearer ";
    let index = header_value.find(token);
    match index {
        Some(idx) => Some(String::from(&header_value[idx + token.len()..])),
        None => None,
    }
}

pub struct PublicRequest<'a> {
    pub state: StateInstance<'a>,
}

impl<'a, 'r> FromRequest<'a, 'r> for PublicRequest<'a> {
    type Error = JsonResponse;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<PublicRequest<'a>, Self::Error> {
        let app = StateInstance::<'a>::from_request(&request);
        match app {
            request::Outcome::Success(state) => Outcome::Success(PublicRequest { state }),
            request::Outcome::Failure(err) => {
                request::Outcome::Failure(create_error(err.0, "Failed to connect to database"))
            }
            request::Outcome::Forward(fwd) => request::Outcome::Forward(fwd),
        }
    }
}

pub struct UserRequest<'a> {
    pub state: StateInstance<'a>,
    pub token: AccessJwtToken,
}

impl<'a, 'r> FromRequest<'a, 'r> for UserRequest<'a> {
    type Error = JsonResponse;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<UserRequest<'a>, Self::Error> {
        let auth_header = request.headers().get_one("Authorization");
        match auth_header {
            Some(auth_token) => {
                let public_request = PublicRequest::from_request(&request);
                match public_request {
                    request::Outcome::Success(public) => {
                        let jwt_string = extract_jwt_from_header(auth_token);
                        match jwt_string {
                            Some(valid_jwt_string) => {
                                let jwt_token = verify_access_token(
                                    valid_jwt_string.as_str(),
                                    &public.state.jwt_key,
                                );
                                match jwt_token {
                                    Some(valid_token) => request::Outcome::Success(UserRequest {
                                        state: public.state,
                                        token: valid_token,
                                    }),
                                    None => request::Outcome::Failure(create_error(
                                        Status::Unauthorized,
                                        "Invalid JWT token",
                                    )),
                                }
                            }
                            None => request::Outcome::Failure(create_error(
                                Status::Unauthorized,
                                "No Bearer token",
                            )),
                        }
                    }
                    request::Outcome::Failure(err) => request::Outcome::Failure(err),
                    request::Outcome::Forward(fwd) => request::Outcome::Forward(fwd),
                }
            }
            _ => Outcome::Failure(create_error(Status::Unauthorized, "No auth credentials")),
        }
    }
}

pub struct AdminRequest<'a>(pub UserRequest<'a>);

impl<'a> Deref for AdminRequest<'a> {
    type Target = UserRequest<'a>;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for AdminRequest<'a> {
    type Error = JsonResponse;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<AdminRequest<'a>, Self::Error> {
        let user_request = UserRequest::from_request(&request);
        match user_request {
            request::Outcome::Success(user) => {
                if user.token.is_admin {
                    request::Outcome::Success(AdminRequest(user))
                } else {
                    request::Outcome::Failure(create_error(
                        Status::Forbidden,
                        "Admin access required",
                    ))
                }
            }
            request::Outcome::Failure(err) => request::Outcome::Failure(err),
            request::Outcome::Forward(fwd) => request::Outcome::Forward(fwd),
        }
    }
}
