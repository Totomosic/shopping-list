use rocket::http::Status;

use crate::{
    auth::{AdminRequest, PublicRequest, UserRequest},
    models::item::{NewShoppingItem, ShoppingItem},
    responses::{error_response, success_response, JsonResponse},
    utils::handle_request,
};

use rocket_contrib::json::Json;

#[get("/items?<q>")]
pub fn get_all_items(
    request: Result<PublicRequest, JsonResponse>,
    q: Option<String>,
) -> JsonResponse {
    handle_request(request, |req| -> JsonResponse {
        match q {
            Some(query) => success_response(json!(ShoppingItem::search_all_items(
                &req.state.connection,
                query.as_str()
            ))),
            None => success_response(json!(ShoppingItem::get_all_items(&req.state.connection))),
        }
    })
}

#[post("/items", data = "<new_item>")]
pub fn post_new_item(
    request: Result<AdminRequest, JsonResponse>,
    new_item: Option<Json<NewShoppingItem>>,
) -> JsonResponse {
    handle_request(request, |req| -> JsonResponse {
        match &new_item {
            Some(item_data) => {
                let result = ShoppingItem::insert_item(
                    &req.state.connection,
                    &NewShoppingItem {
                        name: item_data.name.clone(),
                        description: item_data.description.clone(),
                        image_url: item_data.image_url.clone(),
                        default_unit_type: item_data.default_unit_type.clone(),
                    },
                );
                if result {
                    let item = ShoppingItem::get_last_inserted_item(&req.state.connection);
                    match item {
                        Ok(inserted_item) => success_response(json!(inserted_item)),
                        Err(_) => {
                            error_response(Status::InternalServerError, "Failed to insert item")
                        }
                    }
                } else {
                    error_response(Status::InternalServerError, "Failed to insert item")
                }
            }
            None => error_response(Status::BadRequest, "Failed to parse new item data"),
        }
    })
}

#[delete("/items/<item_id>")]
pub fn delete_item(request: Result<AdminRequest, JsonResponse>, item_id: i32) -> JsonResponse {
    handle_request(request, |req: AdminRequest| -> JsonResponse {
        let potential_item = ShoppingItem::get_item_by_id(&req.state.connection, item_id);
        match potential_item {
            Ok(item) => {
                let success = ShoppingItem::delete_item(&req.state.connection, item_id);
                if success {
                    success_response(json!(item))
                } else {
                    error_response(Status::InternalServerError, "Failed to delete user")
                }
            }
            Err(_) => error_response(Status::NotFound, "Item not found"),
        }
    })
}
