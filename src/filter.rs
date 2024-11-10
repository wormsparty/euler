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
            "equals" => Ok(query.filter(column.eq(self.value))),
            "notEquals" => Ok(query.filter(column.ne(self.value))),
            "contains" => Ok(query.filter(column.contains(self.value))),
            "notContains" => Ok(query.filter(column.contains(self.value).not())),
            "startsWith" => Ok(query.filter(column.starts_with(self.value))),
            "endsWith" => Ok(query.filter(column.ends_with(self.value))),
            "blank" => Ok(query.filter(column.is_null())),
            "notBlank" => Ok(query.filter(column.is_not_null())),
            _ => Err(DbErr::Custom(format!(
                "Opérateur non supporté: {}.",
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