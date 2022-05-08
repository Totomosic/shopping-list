use std::ops::Deref;

use diesel::pg::PgConnection;
use r2d2;
use r2d2_diesel::ConnectionManager;
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Request, State};

use super::models::{ User, NewUser };

pub type ConnectionPool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub struct DatabaseConnection(pub r2d2::PooledConnection<ConnectionManager<PgConnection>>);

pub struct StateInstance<'a> {
  pub connection: DatabaseConnection,
  pub pwd_salt: String,
  pub pwd_config: &'a argon2::Config<'static>
}

impl<'a> StateInstance<'a> {
  pub fn hash_password(&self, password: &str) -> String {
    argon2::hash_encoded(password.as_bytes(), self.pwd_salt.as_bytes(), &self.pwd_config).unwrap()
  }

  pub fn verify_password(&self, hash: &str, password: &str) -> bool {
    argon2::verify_encoded(hash, password.as_bytes()).unwrap()
  }
}

pub struct ApplicationState {
  pub connection_pool: ConnectionPool,
  pub pwd_salt: String,
  pub pwd_config: argon2::Config<'static>,
}

impl ApplicationState {
  pub fn get_instance<'r>(&'r self) -> Result<StateInstance<'r>, ()> {
    let connection = self.connection_pool.get();
    match connection {
      Ok(conn) => Ok(StateInstance { connection: DatabaseConnection(conn), pwd_salt: self.pwd_salt.clone(), pwd_config: &self.pwd_config }),
      Err(_) => Err(())
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
      User::insert_user(NewUser{
        display_name: String::from("Super User"),
        username: username,
        password_hash: state.hash_password(password.as_str()),
        is_admin: true,
      }, &state.connection);
      println!("Created Superuser.")
    },
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
    pwd_config: argon2::Config {
      variant: argon2::Variant::Argon2i,
      version: argon2::Version::Version13,
      mem_cost: 65536,
      time_cost: 10,
      lanes: 4,
      thread_mode: argon2::ThreadMode::Parallel,
      secret: &[],
      ad: &[],
      hash_length: 32,
    },
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
