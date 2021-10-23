use std::collections::HashMap;

use crate::{models::EntryEntity, schema::entries, TimlerDb};
use diesel::prelude::*;
use rocket_dyn_templates::Template;

pub mod entry;
pub mod retrieve;

use crate::utility::EntryContext;

#[get("/")]
pub async fn index(db: TimlerDb) -> Template {
    let result = db
        .run(move |conn| {
            entries::table
                .order(entries::id.desc())
                .first::<EntryEntity>(conn)
        })
        .await;

    match result {
        Ok(entry) => Template::render("index", EntryContext::from(entry)),
        Err(e) => {
            let mut context = HashMap::new();
            context.insert("error", format!("{}", e));

            Template::render("error", context)
        }
    }
}
