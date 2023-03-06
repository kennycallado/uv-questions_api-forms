use diesel::prelude::*;

use crate::config::database::Db;
use crate::database::schema::questions;

use crate::app::modules::question::model::{NewQuestion, Question};

pub async fn get_all(db: &Db) -> Result<Vec<Question>, diesel::result::Error> {
    let result = db
        .run(move |conn| questions::table.load::<(i32, String, String)>(conn))
        .await?;

    let questions: Vec<Question> = result.into_iter().map(|q| q.into()).collect();

    Ok(questions)
}

pub async fn get_by_id(db: &Db, id: i32) -> Result<Question, diesel::result::Error> {
    let question = db
        .run(move |conn| questions::table.find(id).first::<(i32, String, String)>(conn))
        .await?;

    Ok(question.into())
}

pub async fn add(db: &Db, question: NewQuestion) -> Result<Question, diesel::result::Error> {
    let question = db
        .run(move |conn| {
            diesel::insert_into(questions::table)
                .values(&question)
                .get_result::<(i32, String, String)>(conn)
        })
        .await?;

    Ok(question.into())
}

pub async fn update(
    db: &Db,
    id: i32,
    question: NewQuestion,
) -> Result<Question, diesel::result::Error> {
    let question = db
        .run(move |conn| {
            diesel::update(questions::table.find(id))
                .set(&question)
                .get_result::<(i32, String, String)>(conn)
        })
        .await?;

    Ok(question.into())
}
