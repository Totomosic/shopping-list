use crate::auth::AdminRequest;
use crate::models::User;
use crate::responses::{success_response, JsonResponse};

#[get("/users")]
pub fn get_users(request: Result<AdminRequest, JsonResponse>) -> JsonResponse {
    match request {
        Ok(req) => success_response(json!(User::get_all_users(&req.state.connection))),
        Err(res) => res,
    }
}
