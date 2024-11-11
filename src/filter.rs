use std::collections::HashMap;
use std::string::String;
use sea_orm::{ColumnTrait, DbConn, Order, QueryFilter, QueryOrder, QuerySelect};
use sea_orm::{DbErr, EntityTrait};
use serde::Deserialize;
use crate::entities::entity::Column;

#[derive(Deserialize)]
pub struct Sort {
    #[serde(rename = "colId")]
    pub col_id: String,
    pub sort: String,
}

#[derive(Deserialize)]
pub struct Filter {
    #[serde(rename = "filterType")]
    pub filter_type: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub filter: String,
}

#[derive(Deserialize)]
pub struct Query {
    pub start: u64,
    pub end: u64,
    pub filter: HashMap<String, Filter>,
    pub sort: Vec<Sort>,
    #[serde(rename = "globalSearch")]
    pub global_search: String,
}

impl Query {
    pub async fn apply_filters<E: EntityTrait>(
        &self,
        global_searchable_fields: &Vec<&str>,
        column_map: &HashMap<&str, Column>,
        db: &DbConn,
    ) -> Result<Vec<E::Model>, DbErr> {
        let mut qs = E::find();

        // Filter by each field
        for (name, filter) in &self.filter {
            let column= column_map.get(name.as_str()).unwrap();

            if filter.filter_type != "text" {
                return Err(DbErr::Custom("Unsupported filter type".to_string()));
            }

            match filter.kind.as_str() {
                "equals" => qs = qs.filter(column.eq(filter.filter.clone())),
                "notEquals" => qs = qs.filter(column.ne(filter.filter.clone())),
                "contains" => qs = qs.filter(column.contains(filter.filter.clone())),
                "notContains" => qs = qs.filter(column.contains(filter.filter.clone()).not()),
                "startsWith" => qs = qs.filter(column.starts_with(filter.filter.clone())),
                "endsWith" => qs = qs.filter(column.ends_with(filter.filter.clone())),
                "blank" => qs = qs.filter(column.is_null()),
                "notBlank" => qs = qs.filter(column.is_not_null()),
                _ => return Err(DbErr::Custom(format!(
                    "Opérateur non supporté: {}.",
                    filter.kind
                ))),
            }
        }

        // Global filter
        if !self.global_search.is_empty() {
            // This seems to be safe from injection as the builder replaces spaces with underscores
            for field in global_searchable_fields {
                let column= column_map.get(field).unwrap();
                qs = qs.filter(column.contains(&self.global_search));
            }
        }

        // Sorting
        if self.sort.len() > 0 {
            let sort = &self.sort.first().unwrap();
            let column= column_map.get(sort.col_id.as_str()).unwrap();

            qs = qs.order_by(column.clone(), if sort.sort.to_lowercase() == "asc" { Order::Asc } else { Order::Desc });
        }

        // Paging
        qs = qs.offset(self.start).limit(self.end - self.start);

        qs.all(db).await
    }
}