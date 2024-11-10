use std::string::String;
use sea_orm::{ColumnTrait, Iterable, QueryFilter};
use sea_orm::{DbErr, EntityTrait, Select};

#[derive(Debug)]
pub struct DynamicFilter {
    field: String,
    operator: String,
    value: String,
}

impl DynamicFilter {
    pub fn new(field: &str, operator: &str, value: &str) -> Self {
        Self {
            field: field.to_string(),
            operator: operator.to_string(),
            value: value.to_string(),
        }
    }

    pub fn apply_to_query<E: EntityTrait>(
        self,
        query: Select<E>,
    ) -> Result<Select<E>, DbErr> {
        let column = E::Column::iter()
            .find(|col| format!("{:?}", col) == self.field)
            .ok_or_else(|| {
                DbErr::Custom(format!(
                    "Colonne '{}' non trouvée. Les colonnes disponibles sont: {:?}",
                    self.field,
                    E::Column::iter()
                        .map(|c| format!("{:?}", c))
                        .collect::<Vec<_>>()
                ))
            })?;

        match self.operator.as_str() {
            "eq" => Ok(query.filter(column.eq(self.value))),
            "neq" => Ok(query.filter(column.ne(self.value))),
            "gt" => Ok(query.filter(column.gt(self.value))),
            "gte" => Ok(query.filter(column.gte(self.value))),
            "lt" => Ok(query.filter(column.lt(self.value))),
            "lte" => Ok(query.filter(column.lte(self.value))),
            "like" => Ok(query.filter(column.contains(&self.value))),
            "in" => {
                let values: Vec<&str> = self.value.split(',').collect();
                Ok(query.filter(column.is_in(values)))
            }
            _ => Err(DbErr::Custom(format!(
                "Opérateur non supporté: {}. Les opérateurs supportés sont: \
                 eq, neq, gt, gte, lt, lte, like, in",
                self.operator
            ))),
        }
    }
}

/* To debug column names
pub fn print_column_names<E: EntityTrait>() {
    println!("Noms des colonnes avec leur format exact:");
    for col in E::Column::iter() {
        println!("  - Nom exact: '{:?}'", col);
        // Affiche aussi le nom en tant que string pour voir la différence
        println!("    Format string: '{}'", format!("{:?}", col));
    }
}*/