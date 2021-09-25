use serde::{Deserialize, Serialize};
use crate::schema::image_master;

#[derive(Debug, Queryable, Insertable, Deserialize, Serialize)]
#[table_name = "image_master"]
pub struct ImageMaster {
    pub account_id: Option<i32>,
    pub image_base64: Option<String>,
    pub delete_flag: i8
}