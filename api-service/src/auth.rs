use std::ops::Deref;

use crate::db::StateInstance;
use crate::responses::{JsonResponse, error_response};

use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Outcome, Request };

pub fn create_error(status: Status, message: &str) -> (Status, JsonResponse) {
  (status, error_response(status, message))
}

pub struct PublicRequest<'a> {
  pub state: StateInstance<'a>,
}

impl<'a, 'r> FromRequest<'a, 'r> for PublicRequest<'a> {
  type Error = JsonResponse;

  fn from_request(request: &'a Request<'r>) -> request::Outcome<PublicRequest<'a>, Self::Error> {
    let app = StateInstance::<'a>::from_request(&request);
    match app {
      request::Outcome::Success(state) => Outcome::Success(PublicRequest{ state }),
      request::Outcome::Failure(err) => request::Outcome::Failure(create_error(err.0, "Failed to connect to database")),
      request::Outcome::Forward(fwd) => request::Outcome::Forward(fwd),
    }
  }
}

pub struct UserRequest<'a> {
  pub state: StateInstance<'a>,
  pub token: String,
}

impl<'a, 'r> FromRequest<'a, 'r> for UserRequest<'a> {
  type Error = JsonResponse;

  fn from_request(request: &'a Request<'r>) -> request::Outcome<UserRequest<'a>, Self::Error> {
    let auth_header = request.headers().get_one("Authorization");
    match auth_header {
      Some(auth_token) => {
        let public_request = PublicRequest::from_request(&request);
        match public_request {
          request::Outcome::Success(public) => Outcome::Success(UserRequest{ state: public.state, token: String::from(auth_token) }),
          request::Outcome::Failure(err) => request::Outcome::Failure(err),
          request::Outcome::Forward(fwd) => request::Outcome::Forward(fwd),
        }
      },
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
        request::Outcome::Success(AdminRequest(user))
      },
      request::Outcome::Failure(err) => request::Outcome::Failure(err),
      request::Outcome::Forward(fwd) => request::Outcome::Forward(fwd),
    }
  }
}
