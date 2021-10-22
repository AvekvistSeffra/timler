use diesel::prelude::*;
use rocket::response::Redirect;

use crate::{
    models::{Entry, EntryEntity},
    schema::entries,
    utility::{now, today},
    TimlerDb,
};

async fn new_entry(db: TimlerDb) -> Redirect {
    let entry = Entry {
        work_date: today(),
        start_time: now(),
        end_time: None,
    };

    let entry = db
        .run(move |conn| {
            diesel::insert_into(entries::table)
                .values(entry)
                .execute(conn)
        })
        .await;

    match entry {
        Ok(_) => Redirect::to(uri!("/")),
        Err(_) => Redirect::to(uri!("/")),
    }
}

async fn update_entry(db: TimlerDb, id: i32) -> Redirect {
    let entry = db
        .run(move |conn| {
            diesel::update(entries::dsl::entries.find(id))
                .set(entries::end_time.eq(Some(now())))
                .execute(conn)
        })
        .await;

    match entry {
        Ok(_) => Redirect::to(uri!("/")),
        Err(_) => Redirect::to(uri!("/")),
    }
}

#[get("/entry")]
pub async fn handle_entry(db: TimlerDb) -> Redirect {
    let entry = db
        .run(move |conn| {
            entries::table
                .order(entries::id.desc())
                .first::<EntryEntity>(conn)
        })
        .await;

    let entry = match entry {
        Ok(x) => x,
        Err(_) => return new_entry(db).await,
    };

    match entry.end_time {
        Some(_) => new_entry(db).await,
        None => update_entry(db, entry.id).await,
    }
}
