use serde::{Deserialize, Serialize};

use crate::database::schema::questions;

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Identifiable)]
#[serde(crate = "rocket::serde")]
pub struct Question {
    pub id: i32,
    pub q_type: String,
    pub question: String,
}

#[derive(Debug, Deserialize, Serialize, Insertable, AsChangeset)]
#[serde(crate = "rocket::serde")]
#[table_name = "questions"]
pub struct NewQuestion {
    pub q_type: String,
    pub question: String,
}

impl From<Question> for NewQuestion {
    fn from(question: Question) -> Self {
        NewQuestion {
            q_type: question.q_type,
            question: question.question,
        }
    }
}
