use diesel::prelude::*;

use crate::config::database::Db;
use crate::database::schema::forms;

use crate::app::modules::form::model::{Form, NewForm};

pub async fn get_all(db: &Db) -> Result<Vec<Form>, diesel::result::Error> {
    let forms = db.run(move |conn| forms::table.load::<Form>(conn)).await;

    forms
}

pub async fn get_by_id(db: &Db, id: i32) -> Result<Form, diesel::result::Error> {
    let form = db
        .run(move |conn| forms::table.find(id).first::<Form>(conn))
        .await;

    form
}

pub async fn create(db: &Db, new_form: NewForm) -> Result<Form, diesel::result::Error> {
    let form = db
        .run(move |conn| {
            diesel::insert_into(forms::table)
                .values(&new_form)
                .get_result::<Form>(conn)
        })
        .await;

    form
}

pub async fn update(db: &Db, id: i32, new_form: NewForm) -> Result<Form, diesel::result::Error> {
    let form = db
        .run(move |conn| {
            diesel::update(forms::table.find(id))
                .set(&new_form)
                .get_result::<Form>(conn)
        })
        .await;

    form
}
