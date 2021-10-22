#[macro_use]
extern crate rocket;

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate rocket_sync_db_pools;

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
        .mount("/", routes![index, handle_entry])
        .mount("/entries", routes![list, read])
        .attach(TimlerDb::fairing())
}
