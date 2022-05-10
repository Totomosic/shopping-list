use crate::auth::PublicRequest;
use crate::responses::{success_response, JsonResponse};

#[get("/")]
pub fn index(request: Result<PublicRequest, JsonResponse>) -> JsonResponse {
    match request {
        Ok(_req) => success_response(json!({
          "status": "Ok",
        })),
        Err(res) => res,
    }
}
