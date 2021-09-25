use diesel::prelude::*;
use sha3::{Digest, Sha3_256};
use rocket::serde::json::Json as Json;

use crate::db::DbConn;
use crate::schema::account::dsl::*;
use crate::models::account::{Account, QueryAccount};

#[rocket::post("/signin", format="json", data="<data>")]
pub fn signin(data: Json<Account>, conn: DbConn) -> String {
    let request_user = data.0.name;
    let pw_hash = format!("{:x}", Sha3_256::digest(data.0.pw.as_bytes()));

    let record = account.filter(name.eq(&request_user))
        .load::<QueryAccount>(&*conn)
        .expect("Error loading posts");

    let record_pw = &record[0].pw;

    if pw_hash == *record_pw {
        return "success login".to_string()
    } else {
        return "faild login".to_string()
    }
}