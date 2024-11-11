use std::collections::HashMap;
mod entities;
mod filter;

use futures::executor::block_on;
use sea_orm::{Database, DbErr};
use entities::{prelude::*};
use crate::filter::{Query};

const DATABASE_URL: &str = "postgres://rust:rust@localhost:5432/rust";

async fn run() -> Result<(), DbErr> {
    let db = Database::connect(DATABASE_URL).await?;
    let global_searchable = vec!["colonne_1", "colonne_2"];

    let query = Query {
        start: 0,
        end: 100,
        filter: HashMap::new(),
        sort: Vec::new(),
        global_search: "".to_string(),
    };

    let entities = Query::apply_filters::<Entity>(&query, &global_searchable, &db).await?;

    println!("{:?}", entities);
    println!("Done :)");

    Ok(())
}

fn main() {
    if let Err(err) = block_on(run()) {
        panic!("{}", err);
    }

    println!("Done!");
}