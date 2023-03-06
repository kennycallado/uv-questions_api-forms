use std::fmt;

use diesel::{types::FromSqlRow, sql_types::Text, pg::Pg, row::Row, expression::AsExpression, helper_types::AsExprOf};
use serde::{Deserialize, Serialize};

use crate::database::schema::questions;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum TypeEnum {
    Range,
    Input,
}

impl fmt::Display for TypeEnum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                TypeEnum::Range => "range",
                TypeEnum::Input => "input",
            }
        )
    }
}

impl FromSqlRow<Text, Pg> for TypeEnum {
    fn build_from_row<R: Row<Pg>>(
        row: &mut R,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        match String::build_from_row(row)?.as_ref() {
            "range" => Ok(TypeEnum::Range),
            "input" => Ok(TypeEnum::Input),
            v => Err(format!("Unknown value {} for TypeEnum found", v).into()),
        }
    }
}

impl AsExpression<Text> for TypeEnum {
    type Expression = AsExprOf<String, Text>;
    fn as_expression(self) -> Self::Expression {
        <String as AsExpression<Text>>::as_expression(self.to_string())
    }
}

impl<'a> AsExpression<Text> for &'a TypeEnum {
    type Expression = AsExprOf<String, Text>;
    fn as_expression(self) -> Self::Expression {
        <String as AsExpression<Text>>::as_expression(self.to_string())
    }
}

#[derive(Debug, Deserialize, Serialize, Queryable, Identifiable)]
#[serde(crate = "rocket::serde")]
pub struct Question {
    pub id: i32,
    pub q_type: TypeEnum,
    pub question: String,
}

impl From<(i32, String, String)> for Question {
    fn from(question: (i32, String, String)) -> Self {
        Question {
            id: question.0,
            q_type: match question.1.as_ref() {
                "range" => TypeEnum::Range,
                "input" => TypeEnum::Input,
                _ => TypeEnum::Range,
            },
            question: question.2,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Insertable, AsChangeset)]
#[serde(crate = "rocket::serde")]
#[table_name = "questions"]
pub struct NewQuestion {
    pub q_type: TypeEnum,
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
