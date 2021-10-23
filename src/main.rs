#[macro_use]
extern crate rocket;

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate serde;

#[macro_use]
extern crate rocket_sync_db_pools;

use rocket::fs::FileServer;
use rocket_dyn_templates::Template;

mod models;
mod routes;
mod schema;
mod utility;

use routes::entry::handle_entry;
use routes::index;
use routes::retrieve::{list, read};

#[database("timler")]
pub struct TimlerDb(diesel::MysqlConnection);

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, handle_entry, list, read])
        .mount("/public", FileServer::from("static/"))
        .attach(TimlerDb::fairing())
        .attach(Template::fairing())
}
