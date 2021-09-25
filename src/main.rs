#![feature(proc_macro_hygiene)]
#![feature(decl_macro)]


#[macro_use] 
extern crate rocket;
#[macro_use] 
extern crate diesel;
use dotenv::dotenv;


mod db;
mod schema;
mod pages;
mod models;

#[launch]
fn rocket() -> _ {
    dotenv().ok();
    
    rocket::build()
        .manage(db::init_pool())
        .mount("/", routes![
            pages::signup::signup,
            pages::signin::signin,
            pages::image_save::regist_image
        ])
}