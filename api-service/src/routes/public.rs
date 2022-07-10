use crate::auth::PublicRequest;
use crate::responses::{success_response, JsonResponse};
use crate::utils::handle_request;

#[get("/")]
pub fn index(request: Result<PublicRequest, JsonResponse>) -> JsonResponse {
    handle_request(request, |_| -> JsonResponse {
        success_response(json!({
          "status": "Ok",
        }))
    })
}
