use std::env;
use std::ops::Deref;

use diesel::mysql::MysqlConnection;
use r2d2;
use r2d2_diesel::ConnectionManager;
use rocket::State;
use rocket::http::Status;
use rocket::request::{self, Request, FromRequest};


type Pool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

pub fn init_pool() -> Pool {
    let manager = ConnectionManager::<MysqlConnection>::new(database_url());
    Pool::new(manager).expect("db pool")
}

fn database_url() -> String {
    env::var("DATABASE_URL").expect("DATABASE_URL must be set")
}

pub struct DbConn(pub r2d2::PooledConnection<ConnectionManager<MysqlConnection>>);

#[rocket::async_trait]
impl <'r> FromRequest<'r> for DbConn {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let pool = request.guard::<&State<Pool>>().await;

        match pool.unwrap().inner().get() {
            Ok(conn) => request::Outcome::Success(DbConn(conn)),
            Err(_) => request::Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}

impl Deref for DbConn {
    type Target = MysqlConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}