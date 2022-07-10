use crate::auth::{
    generate_access_token, generate_refresh_token, verify_refresh_token, PublicRequest,
};
use crate::models::auth::User;
use crate::responses::{error_response, success_response, JsonResponse};
use crate::utils::handle_request;
use rocket::http::Status;
use rocket_contrib::json::Json;
use serde_derive::Deserialize;

#[derive(Deserialize)]
pub struct BasicAuth {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct JwtRefresh {
    pub token: String,
}

#[post("/core/auth/basic", data = "<auth>")]
pub fn basic_auth(
    request: Result<PublicRequest, JsonResponse>,
    auth: Option<Json<BasicAuth>>,
) -> JsonResponse {
    handle_request(request, |req| -> JsonResponse {
        match &auth {
            Some(body) => {
                let potential_user =
                    User::get_user_by_username(&req.state.connection, body.username.as_str());
                match potential_user {
                    Ok(user) => {
                        if req
                            .state
                            .verify_password(user.password_hash.as_str(), body.password.as_str())
                        {
                            success_response(json!({
                                "refresh_token": generate_refresh_token(&user, &req.state.jwt_key),
                                "access_token": generate_access_token(&user, &req.state.jwt_key),
                            }))
                        } else {
                            error_response(Status::Forbidden, "Wrong password")
                        }
                    }
                    Err(_) => error_response(Status::Forbidden, "Username not found."),
                }
            }
            None => error_response(Status::BadRequest, "Invalid body data"),
        }
    })
}

#[post("/core/auth/refresh", data = "<refresh>")]
pub fn refresh_jwt(
    request: Result<PublicRequest, JsonResponse>,
    refresh: Option<Json<JwtRefresh>>,
) -> JsonResponse {
    handle_request(request, |req| -> JsonResponse {
        match refresh {
            Some(refresh_token) => {
                let token = verify_refresh_token(&refresh_token.token, &req.state.jwt_key);
                match token {
                    Some(tok) => match User::get_user_by_id(&req.state.connection, tok.user_id) {
                        Ok(user) => {
                            let access_token = generate_access_token(&user, &req.state.jwt_key);
                            if !access_token.is_empty() {
                                success_response(json!({
                                    "token": access_token,
                                }))
                            } else {
                                error_response(
                                    Status::InternalServerError,
                                    "Failed to generate token",
                                )
                            }
                        }
                        Err(_) => error_response(Status::Forbidden, "No user found."),
                    },
                    None => error_response(Status::Forbidden, "Invalid JWT"),
                }
            }
            None => error_response(Status::BadRequest, "Failed to parse body"),
        }
    })
}
