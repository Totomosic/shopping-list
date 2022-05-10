use rocket::http::ContentType;
use rocket::http::Status;
use rocket::request::Request;
use rocket::response::{self, Responder, Response};
use serde_json::Value;
use std::io::Cursor;

#[derive(Debug)]
pub struct JsonResponse {
    status: Status,
    message: Value,
}

impl<'r> Responder<'r> for JsonResponse {
    fn respond_to(self, _: &Request) -> response::Result<'r> {
        Ok(Response::build()
            .status(self.status)
            .header(ContentType::JSON)
            .sized_body(Cursor::new(self.message.to_string()))
            .finalize())
    }
}

pub fn success_response(data: Value) -> JsonResponse {
    JsonResponse {
        status: Status::Ok,
        message: json!({
          "success": true,
          "error": json!(null),
          "data": data,
        }),
    }
}

pub fn error_response(status: Status, error: &str) -> JsonResponse {
    JsonResponse {
        status: status,
        message: json!({
          "success": false,
          "error": error,
          "data": json!(null),
        }),
    }
}
