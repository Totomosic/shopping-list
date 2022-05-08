use super::auth::{ PublicRequest };
use super::responses::{ success_response, JsonResponse };
use super::models::User;

#[get("/")]
pub fn index(request: Result<PublicRequest, JsonResponse>) -> JsonResponse {
  match request {
    Ok(_req) => success_response(json!({
      "status": "Ok",
    })),
    Err(res) => res
  }
}

#[get("/users")]
pub fn get_users(request: Result<PublicRequest, JsonResponse>) -> JsonResponse {
  match request {
    Ok(req) => success_response(json!(User::get_all_users(&req.state.connection))),
    Err(res) => res
  }
}
