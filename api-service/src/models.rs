use super::schema::users;
use super::schema::users::dsl::users as all_users;
use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use serde::Deserialize;
use serde_derive::Serialize;

#[derive(Debug, Serialize, Queryable)]
pub struct User {
    pub id: i32,
    pub display_name: String,
    #[serde(skip_serializing)]
    pub username: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub is_admin: bool,
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub display_name: String,
    pub username: String,
    pub password_hash: String,
    pub is_admin: bool,
}

impl User {
    pub fn get_all_users(conn: &PgConnection) -> Vec<User> {
        all_users
            .order(users::id.desc())
            .load::<User>(conn)
            .expect("Error loading users")
    }

    pub fn get_user_by_id(id: i32, conn: &PgConnection) -> Result<User, diesel::result::Error> {
        all_users.find(id).first::<User>(conn)
    }

    pub fn get_user_by_username(
        username: &str,
        conn: &PgConnection,
    ) -> Result<User, diesel::result::Error> {
        all_users
            .filter(users::username.eq(username))
            .first::<User>(conn)
    }

    pub fn insert_user(user: &NewUser, conn: &PgConnection) -> bool {
        diesel::insert_into(users::table)
            .values(user)
            .execute(conn)
            .is_ok()
    }

    pub fn delete_user(user_id: i32, conn: &PgConnection) -> bool {
        diesel::delete(users::table)
            .filter(users::id.eq(user_id))
            .execute(conn)
            .is_ok()
    }

    pub fn delete_all_admins(conn: &PgConnection) -> bool {
        diesel::delete(users::table)
            .filter(users::is_admin.eq(true))
            .execute(conn)
            .is_ok()
    }

    pub fn get_last_inserted_user(conn: &PgConnection) -> Result<User, diesel::result::Error> {
        all_users.order(users::id.desc()).first::<User>(conn)
    }
}
