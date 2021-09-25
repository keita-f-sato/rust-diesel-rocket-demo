use diesel::prelude::*;
use rocket::serde::json::Json as Json;

use crate::db::DbConn;
use crate::schema::image_master;
use crate::models::image_master::ImageMaster;

#[post("/image-save", format="json", data="<data>")]
pub fn regist_image(data: Json<ImageMaster>, conn: DbConn) -> String {
    let insert_image_data = ImageMaster {
        account_id: data.0.account_id,
        image_base64: data.0.image_base64,
        delete_flag: 0
    };

    let res = diesel::insert_into(image_master::table)
        .values(&insert_image_data)
        .execute(&*conn);

    
    match res {
        Ok(_) => return "success save image".to_string(),
        Err(_) => return "faild save image".to_string()
    }    
}