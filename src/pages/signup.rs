use diesel::prelude::*;
use sha3::{Digest, Sha3_256};
use rocket::serde::json::Json as Json;

use crate::db::DbConn;
use crate::schema::account::dsl::*;

use crate::schema::account;
use crate::models::account::{Account, QueryAccount};

#[rocket::post("/signup", format="json", data="<data>")]
pub fn signup(data: Json<Account>, conn: DbConn) -> String {
    let request_name = data.0.name;

    let new_account = Account {
        name: request_name,
        pw: format!("{:x}", Sha3_256::digest(data.0.pw.as_bytes()))
    };

    let record = account.filter(name.eq(&new_account.name))
        .load::<QueryAccount>(&*conn)
        .expect("Error loading posts");

    let length_record = record.len();

    println!("{:?}", length_record >= 1);

    if length_record >= 1 {
        println!("faild account");
        return "aready account name".to_string()
    } else {
        let res = diesel::insert_into(account::table)
            .values(new_account)
            .execute(&*conn);

        match res {
            Ok(_) => return "success create".to_string(),
            Err(_) => return "faild create".to_string()
        }
    }
}