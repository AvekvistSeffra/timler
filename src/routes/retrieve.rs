use diesel::prelude::*;

use crate::{models::EntryEntity, schema::entries, TimlerDb};

#[get("/<id>")]
pub async fn read(db: TimlerDb, id: i32) -> String {
    match db
        .run(move |conn| {
            entries::table
                .filter(entries::id.eq(id))
                .first::<EntryEntity>(conn)
        })
        .await
    {
        Ok(x) => format!("{}", x),
        Err(e) => format!("Entry with id {} doesn't exist. \nError: {}", id, e),
    }
}

#[get("/")]
pub async fn list(db: TimlerDb) -> String {
    let result = db
        .run(move |conn| {
            entries::table
                .order(entries::id.desc())
                .limit(20)
                .load::<EntryEntity>(conn)
                .expect("Failed to load entries")
        })
        .await;

    if result.len() <= 0 {
        return String::from("Couldn't find any entries. ");
    }

    let mut return_string = String::new();

    for entry in result {
        return_string.push_str(&format!("{}\n", entry))
    }

    return_string
}
