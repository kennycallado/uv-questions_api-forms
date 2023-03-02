use serde::{Deserialize, Serialize};

use crate::{app::modules::question::model::Question, database::schema::forms};

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Identifiable, FromForm)]
#[serde(crate = "rocket::serde")]
pub struct Form {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
}

impl From<FormWithQuestions> for Form {
    fn from(form: FormWithQuestions) -> Self {
        Form {
            id: form.id,
            name: form.name,
            description: form.description,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Insertable, AsChangeset)]
#[serde(crate = "rocket::serde")]
#[table_name = "forms"]
pub struct NewForm {
    pub name: String,
    pub description: Option<String>,
}

impl From<Form> for NewForm {
    fn from(form: Form) -> Self {
        NewForm {
            name: form.name,
            description: form.description,
        }
    }
}

impl From<FormWithQuestions> for NewForm {
    fn from(form: FormWithQuestions) -> Self {
        NewForm {
            name: form.name,
            description: form.description,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct FormWithQuestions {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub questions: Vec<Question>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct NewFormWithQuestions {
    pub name: String,
    pub description: Option<String>,
    pub questions_id: Option<Vec<i32>>,
}

impl From<NewFormWithQuestions> for NewForm {
    fn from(form: NewFormWithQuestions) -> Self {
        NewForm {
            name: form.name,
            description: form.description,
        }
    }
}

// impl From<NewFormExpanded> for NewForm {
//     fn from(form: NewFormExpanded) -> Self {
//         NewForm {
//             name: form.name,
//             description: form.description,
//         }
//     }
// }
