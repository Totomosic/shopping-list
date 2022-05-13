use std::ops::Deref;

use diesel::pg::PgConnection;
use hmac::{Hmac, Mac};
use r2d2;
use r2d2_diesel::ConnectionManager;
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Request, State};
use sha2::Sha256;

use super::models::{NewUser, User};

pub type HmacSha256 = Hmac<Sha256>;

pub type ConnectionPool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub struct DatabaseConnection(pub r2d2::PooledConnection<ConnectionManager<PgConnection>>);

pub struct StateInstance<'a> {
    pub connection: DatabaseConnection,
    pub pwd_salt: String,
    pub pwd_config: &'a argon2::Config<'static>,
    pub jwt_key: HmacSha256,
}

impl<'a> StateInstance<'a> {
    pub fn hash_password(&self, password: &str) -> String {
        argon2::hash_encoded(
            password.as_bytes(),
            self.pwd_salt.as_bytes(),
            &self.pwd_config,
        )
        .unwrap()
    }

    pub fn verify_password(&self, hash: &str, password: &str) -> bool {
        let now = std::time::Instant::now();
        let result = argon2::verify_encoded(hash, password.as_bytes()).unwrap();
        println!("Elapsed {}", now.elapsed().as_millis());
        result
    }
}

pub struct ApplicationState {
    pub connection_pool: ConnectionPool,
    pub pwd_salt: String,
    pub pwd_config: argon2::Config<'static>,
    pub jwt_key: HmacSha256,
}

impl ApplicationState {
    pub fn get_instance<'r>(&'r self) -> Result<StateInstance<'r>, ()> {
        let connection = self.connection_pool.get();
        match connection {
            Ok(conn) => Ok(StateInstance {
                connection: DatabaseConnection(conn),
                pwd_salt: self.pwd_salt.clone(),
                pwd_config: &self.pwd_config,
                jwt_key: self.jwt_key.clone(),
            }),
            Err(_) => Err(()),
        }
    }
}

fn create_super_user(state: &StateInstance) {
    let username = std::env::var("SUPER_USER_USERNAME").expect("SUPER_USER_USERNAME not found.");
    let password = std::env::var("SUPER_USER_PASSWORD").expect("SUPER_USER_PASSWORD not found.");
    let existing_user = User::get_user_by_username(&username.as_str(), &state.connection);
    match existing_user {
        Ok(_) => println!("Superuser already exists."),
        Err(_) => {
            // Create here
            User::delete_all_admins(&state.connection);
            User::insert_user(
                &NewUser {
                    display_name: String::from("Super User"),
                    username,
                    password_hash: state.hash_password(password.as_str()),
                    is_admin: true,
                },
                &state.connection,
            );
            println!("Created Superuser.")
        }
    };
}

pub fn init_pool(db_url: String) -> ApplicationState {
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    let pool = r2d2::Pool::builder()
        .max_size(1)
        .build(manager)
        .expect("DB pool creation failure");
    let state = ApplicationState {
        connection_pool: pool,
        pwd_salt: String::from("SaltSaltSaltSalt"),
        pwd_config: argon2::Config::default(),
        jwt_key: HmacSha256::new_from_slice(b"secret-key").unwrap(),
    };
    create_super_user(&state.get_instance().unwrap());
    state
}

impl<'a, 'r> FromRequest<'a, 'r> for StateInstance<'a> {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<StateInstance<'a>, Self::Error> {
        let state = request.guard::<State<ApplicationState>>();
        match state {
            request::Outcome::Success(st) => match st.inner().get_instance() {
                Ok(inst) => request::Outcome::Success(inst),
                Err(_) => request::Outcome::Failure((Status::ServiceUnavailable, ())),
            },
            request::Outcome::Failure(err) => request::Outcome::Failure(err),
            request::Outcome::Forward(fwd) => request::Outcome::Forward(fwd),
        }
    }
}

impl Deref for DatabaseConnection {
    type Target = PgConnection;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
