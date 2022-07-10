use crate::schema::shopping_item;
use crate::schema::shopping_item::dsl::shopping_item as all_items;
use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use serde::Deserialize;
use serde_derive::Serialize;

use diesel::sql_types::VarChar;
use diesel_enum::DbEnum;

#[derive(Debug)]
pub struct UnitTypeError {
    pub msg: String,
    pub status: u16,
}

impl UnitTypeError {
    fn not_found(msg: String) -> Self {
        Self { msg, status: 404 }
    }
}

#[derive(
    Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, AsExpression, FromSqlRow, DbEnum,
)]
#[sql_type = "VarChar"]
#[error_fn = "UnitTypeError::not_found"]
#[error_type = "UnitTypeError"]
pub enum UnitType {
    Count,
    Mass,
    Capacity,
}

#[derive(Debug, Serialize, Queryable)]
pub struct ShoppingItem {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub image_url: Option<String>,
    pub default_unit_type: UnitType,
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[table_name = "shopping_item"]
pub struct NewShoppingItem {
    pub name: String,
    pub description: Option<String>,
    pub image_url: Option<String>,
    pub default_unit_type: UnitType,
}

impl ShoppingItem {
    pub fn get_all_items(conn: &PgConnection) -> Vec<ShoppingItem> {
        all_items
            .order(shopping_item::id.desc())
            .load::<ShoppingItem>(conn)
            .expect("Error loading items")
    }

    pub fn search_all_items(conn: &PgConnection, query: &str) -> Vec<ShoppingItem> {
        all_items
            .order(shopping_item::id.desc())
            .filter(shopping_item::name.ilike(["%", &query, "%"].join("")))
            .load::<ShoppingItem>(conn)
            .expect("Error loading items")
    }

    pub fn get_item_by_id(
        conn: &PgConnection,
        id: i32,
    ) -> Result<ShoppingItem, diesel::result::Error> {
        all_items.find(id).first::<ShoppingItem>(conn)
    }

    pub fn insert_item(conn: &PgConnection, item: &NewShoppingItem) -> bool {
        diesel::insert_into(shopping_item::table)
            .values(item)
            .execute(conn)
            .is_ok()
    }

    pub fn delete_item(conn: &PgConnection, id: i32) -> bool {
        diesel::delete(shopping_item::table)
            .filter(shopping_item::id.eq(id))
            .execute(conn)
            .is_ok()
    }

    pub fn get_last_inserted_item(
        conn: &PgConnection,
    ) -> Result<ShoppingItem, diesel::result::Error> {
        all_items
            .order(shopping_item::id.desc())
            .first::<ShoppingItem>(conn)
    }
}
