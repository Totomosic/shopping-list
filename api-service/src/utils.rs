use crate::responses::JsonResponse;

pub fn handle_request<T, F>(request: Result<T, JsonResponse>, handler: F) -> JsonResponse
where
    F: FnOnce(T) -> JsonResponse,
{
    match request {
        Ok(req) => handler(req),
        Err(err) => err,
    }
}
