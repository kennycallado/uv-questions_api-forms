use diesel::prelude::*;

use crate::app::modules::form_questions::model::NewFormQuestion;
use crate::config::database::Db;

use crate::database::schema::form_questions;
use crate::database::schema::questions;

use crate::app::modules::question::model::Question;

pub async fn get_questions_by_form_id(
    db: &Db,
    form_id: i32,
) -> Result<Vec<Question>, diesel::result::Error> {
    let result = db
        .run(move |conn| {
            form_questions::table
                .inner_join(questions::table)
                .filter(form_questions::form_id.eq(form_id.clone()))
                .select(questions::all_columns)
                .load::<(i32, String, String)>(conn)
        })
        .await?;

    let questions: Vec<Question> = result.into_iter().map(|q| q.into()).collect();

    Ok(questions.into())
}

pub async fn add_questions_to_form(db: &Db, form_id: i32, questions_id: Vec<i32>) -> Result<Vec<Question>, diesel::result::Error> {
    let mut vector = vec![]; 

    for i in questions_id {
        let nfq = NewFormQuestion {
            form_id: form_id.clone(),
            question_id: i
        };

        vector.push(nfq);
    }

    db.run(move |conn| {
            diesel::insert_into(form_questions::table)
                .values(vector)
                .execute(conn)
        }).await?;

    let questions = get_questions_by_form_id(db, form_id).await?;
    Ok(questions)
}

pub async fn update_questions_in_form(db: &Db, form_id: i32, questions: Vec<i32>) -> Result<Vec<Question>, diesel::result::Error> {
    db.run(move |conn| {
            diesel::delete(form_questions::table.filter(form_questions::form_id.eq(form_id.clone())))
                .execute(conn)
        }).await?;

    let questions = add_questions_to_form(db, form_id, questions).await?;

    Ok(questions)
}
