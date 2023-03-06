use serde::{Deserialize, Serialize};

use crate::database::schema::form_questions;

use crate::app::modules::form::model::Form;
use crate::app::modules::question::model::Question;

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Identifiable, Associations)]
#[belongs_to(Form)]
#[belongs_to(Question)]
#[serde(crate = "rocket::serde")]
pub struct FormQuestion {
    pub id: i32,
    pub form_id: i32,
    pub question_id: i32,
}

#[derive(Debug, Deserialize, Serialize, Insertable, AsChangeset)]
#[serde(crate = "rocket::serde")]
#[table_name = "form_questions"]
pub struct NewFormQuestion {
    pub form_id: i32,
    pub question_id: i32,
}

impl From<FormQuestion> for NewFormQuestion {
    fn from(form_question: FormQuestion) -> Self {
        NewFormQuestion {
            form_id: form_question.form_id,
            question_id: form_question.question_id,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct FormQuestionExpanded {
    pub id: i32,
    pub form: Form,
    pub questions: Vec<Question>,
}

// impl From<FormQuestionExpanded> for NewFormQuestion {
//     fn from(form_question: FormQuestionExpanded) -> Self {
//         NewFormQuestion {
//             question_id: form_question.question.id,
//             form_id: form_question.form.id,
//         }
//     }
// }

// impl From<FormQuestionExpanded> for FormQuestion {
//     fn from(form_question: FormQuestionExpanded) -> Self {
//         FormQuestion {
//             id: form_question.id,
//             question_id: form_question.question.id,
//             form_id: form_question.form.id,
//         }
//     }
// }
