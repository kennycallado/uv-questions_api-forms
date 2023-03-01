// use diesel::{types::FromSql, sql_types::{Text, Integer}, backend::Backend, deserialize};
use serde::{Deserialize, Serialize};

use crate::database::schema::forms;

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Identifiable, FromForm)]
#[serde(crate = "rocket::serde")]
pub struct Form {
    id: i32,
    name: String,
    description: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Insertable, AsChangeset)]
#[serde(crate = "rocket::serde")]
#[table_name = "forms"]
pub struct NewForm {
    name: String,
    description: Option<String>,
}

impl From<Form> for NewForm {
    fn from(form: Form) -> Self {
        NewForm {
            name: form.name,
            description: form.description,
        }
    }
}
