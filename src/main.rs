use sea_orm::{DbConn, EntityTrait};
mod entities;
mod filter;

use futures::executor::block_on;
use sea_orm::{Database, DbErr};
use entities::{prelude::*};
use crate::filter::DynamicFilter;

const DATABASE_URL: &str = "postgres://rust:rust@localhost:5432/rust";

pub async fn apply_filters<E: EntityTrait>(
    filters: Vec<DynamicFilter>,
    db: &DbConn,
) -> Result<Vec<E::Model>, DbErr> {
    let mut query = E::find();

    for filter in filters {
        query = filter.apply_to_query(query)?;
    }

    query.all(db).await
}

async fn run() -> Result<(), DbErr> {
    let db = Database::connect(DATABASE_URL).await?;

    let filters = vec![
        DynamicFilter::new("Colonne1", "like", "2"),
        DynamicFilter::new("Colonne2", "like", "3"),
    ];

    let entities = apply_filters::<Entity>(filters, &db).await?;

    println!("{:?}", entities);
    Ok(())
}

fn main() {
    if let Err(err) = block_on(run()) {
        panic!("{}", err);
    }

    println!("Done!");
}