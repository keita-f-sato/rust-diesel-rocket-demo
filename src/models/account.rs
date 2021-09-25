use serde::{Deserialize, Serialize};

use crate::schema::account;

#[derive(Debug, Insertable, Deserialize, Serialize)]
#[table_name = "account"]
pub struct Account {
    pub name: String,
    pub pw: String
}

#[derive(Debug, Queryable, Deserialize, Serialize)]
pub struct QueryAccount {
    pub name_id: i32,
    pub name: String,
    pub pw: String
}


