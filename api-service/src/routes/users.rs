use crate::auth::AdminRequest;
use crate::models::{NewUser, User};
use crate::responses::{error_response, success_response, JsonResponse};
use rocket::http::Status;
use rocket_contrib::json::Json;
use serde_derive::Deserialize;

#[derive(Deserialize)]
pub struct NewUserData {
    pub display_name: String,
    pub username: String,
    pub password: String,
    pub is_admin: bool,
}

#[get("/users")]
pub fn get_users(request: Result<AdminRequest, JsonResponse>) -> JsonResponse {
    match request {
        Ok(req) => success_response(json!(User::get_all_users(&req.state.connection))),
        Err(res) => res,
    }
}

#[post("/users", data = "<new_user>")]
pub fn post_new_user(
    request: Result<AdminRequest, JsonResponse>,
    new_user: Option<Json<NewUserData>>,
) -> JsonResponse {
    match request {
        Ok(req) => match new_user {
            Some(user_data) => {
                let result = User::insert_user(
                    &NewUser {
                        display_name: user_data.display_name.clone(),
                        username: user_data.username.clone(),
                        password_hash: req.state.hash_password(user_data.password.as_str()),
                        is_admin: user_data.is_admin,
                    },
                    &req.state.connection,
                );
                if result {
                    let user = User::get_last_inserted_user(&req.state.connection);
                    match user {
                        Ok(inserted_user) => success_response(json!(inserted_user)),
                        Err(_) => {
                            error_response(Status::InternalServerError, "Failed to insert user")
                        }
                    }
                } else {
                    error_response(Status::InternalServerError, "Failed to insert user")
                }
            }
            None => error_response(Status::BadRequest, "Failed to parse new user data"),
        },
        Err(res) => res,
    }
}

#[delete("/users/<user_id>")]
pub fn delete_user(request: Result<AdminRequest, JsonResponse>, user_id: i32) -> JsonResponse {
    match request {
        Ok(req) => {
            if user_id == req.token.user_id {
                error_response(Status::BadRequest, "Cannot delete self")
            } else {
                let potential_user = User::get_user_by_id(user_id, &req.state.connection);
                match potential_user {
                    Ok(user) => {
                        let success = User::delete_user(user_id, &req.state.connection);
                        if success {
                            success_response(json!(user))
                        } else {
                            error_response(Status::InternalServerError, "Failed to delete user")
                        }
                    }
                    Err(_) => error_response(Status::NotFound, "User not found"),
                }
            }
        }
        Err(res) => res,
    }
}
