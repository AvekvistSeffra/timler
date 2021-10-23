use std::collections::HashMap;

use diesel::prelude::*;
use rocket_dyn_templates::Template;

use crate::{models::EntryEntity, schema::entries, utility::EntryContext, TimlerDb};

#[get("/entries/<id>")]
pub async fn read(db: TimlerDb, id: i32) -> Template {
    let result = db
        .run(move |conn| {
            entries::table
                .filter(entries::id.eq(id))
                .first::<EntryEntity>(conn)
        })
        .await;

    match result {
        Ok(entry) => Template::render("entry", EntryContext::from(entry)),
        Err(e) => {
            let mut context = HashMap::new();
            context.insert("error", format!("{}", e));
            context.insert("id", format!("{}", id));

            Template::render("error", context)
        }
    }
}

#[get("/entries")]
pub async fn list(db: TimlerDb) -> Template {
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
        let mut context = HashMap::new();
        context.insert("error", String::from("No entries"));

        return Template::render("error", context);
    }

    let mut context: HashMap<&str, Vec<EntryContext>> = HashMap::new();
    context.insert(
        "entries",
        result.iter().map(|x| EntryContext::from(*x)).collect(),
    );

    Template::render("entries", context)
}
