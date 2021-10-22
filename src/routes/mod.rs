use crate::{models::EntryEntity, schema::entries, TimlerDb};
use diesel::prelude::*;

pub mod entry;
pub mod retrieve;

#[get("/")]
pub async fn index(db: TimlerDb) -> String {
    let result = db
        .run(move |conn| {
            entries::table
                .order(entries::id.desc())
                .first::<EntryEntity>(conn)
        })
        .await;

    match result {
        Ok(x) => format!("Latest entry: {}", x),
        Err(e) => format!("Couldn't fetch entry. \nError: {}", e),
    }
}
