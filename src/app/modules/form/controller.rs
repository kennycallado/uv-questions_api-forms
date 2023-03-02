use rocket::http::Status;
use rocket::serde::json::Json;

use crate::app::modules::question::model::Question;
use crate::app::providers::guards::admin::AdminClaims;
use crate::app::providers::guards::coord::CoordClaims;
use crate::app::providers::guards::thera::TheraClaims;
use crate::app::providers::guards::user::UserClaims;
use crate::config::database::Db;

use crate::app::modules::form::model::{Form, NewForm};
use crate::app::modules::form::services::repostitory as form_repository;
use crate::app::modules::form_questions::services::respository as form_question_repository;

use super::model::{FormWithQuestions, NewFormWithQuestions};

pub fn routes() -> Vec<rocket::Route> {
    routes![
        get_index_user,
        get_index_none,
        get_show_user,
        get_show_none,
        post_create_admin,
        post_create_coord,
        post_create_thera,
        post_create_none,
        put_update_admin,
        put_update_coord,
        put_update_thera,
        put_update_none,
        // delete_destroy_admin,
        // delete_destroy_coord,
        // delete_destroy_thera,
        // delete_destroy_none
    ]
}

#[get("/", rank = 4)]
pub async fn get_index_user(db: Db, _claims: UserClaims) -> Result<Json<Vec<Form>>, Status> {
    let forms = form_repository::get_all(&db).await;

    match forms {
        Ok(forms) => Ok(Json(forms)),
        Err(e) => {
            println!("Module: form - Controller: get_index_user");
            println!("Error: {:?}", e);
            Err(Status::InternalServerError)
        }
    }
}

#[get("/", rank = 5)]
pub async fn get_index_none() -> Status {
    println!("Module: form - Controller: get_index_none");
    Status::Unauthorized
}

#[get("/<id>", rank = 4)]
pub async fn get_show_user(
    db: Db,
    _claims: UserClaims,
    id: i32,
) -> Result<Json<FormWithQuestions>, Status> {
    let form = form_repository::get_by_id(&db, id).await;
    if let Err(e) = form {
        println!("Module: form - Controller: get_show_user");
        println!("Error: {:?}", e);
        return Err(Status::InternalServerError);
    }
    let form = form.unwrap();

    let questions = form_question_repository::get_questions_by_form_id(&db, id).await;
    if let Err(e) = questions {
        println!("Module: form - Controller: get_show_user");
        println!("Error: {:?}", e);
        return Err(Status::InternalServerError);
    }
    let questions = questions.unwrap();

    let form_expanded = FormWithQuestions {
        id: form.id,
        name: form.name,
        description: form.description,
        questions,
    };

    Ok(Json(form_expanded))
}

#[get("/<_id>", rank = 5)]
pub async fn get_show_none(_id: i32) -> Status {
    println!("Module: form - Controller: get_show_none");
    Status::Unauthorized
}

async fn create_helper(db: &Db, form: NewFormWithQuestions) -> Result<(Form, Vec<Question>), Status> {
    // Get the questions
    let questions = match form.questions_id.clone() {
        Some(questions) => questions,
        None => vec![],
    };

    // Create the form
    let new_form: NewForm = form.into();
    let form = form_repository::create(&db, new_form).await;
    if let Err(e) = form {
        println!("Module: form - Controller: helper");
        println!("Error: {:?}", e);
        return Err(Status::InternalServerError);
    }
    let form = form.unwrap();

    // Add the questions to the form
    let questions = form_question_repository::add_questions_to_form(&db, form.id, questions).await;
    if let Err(e) = questions {
        println!("Module: form - Controller: helper");
        println!("Error: {:?}", e);
        return Err(Status::InternalServerError);
    }
    let questions = questions.unwrap();

    return Ok((form, questions));
}

#[post("/", data = "<new_form>", rank = 1)]
pub async fn post_create_admin(db: Db, _claims: AdminClaims, new_form: Json<NewFormWithQuestions>) -> Result<Json<FormWithQuestions>, Status> {
    let form = new_form.into_inner();

    let (form, questions) = create_helper(&db, form).await?;

    // Prepare the response
    let form_expanded = FormWithQuestions {
        id: form.id,
        name: form.name,
        description: form.description,
        questions,
    };

    Ok(Json(form_expanded))
}

#[post("/", data = "<new_form>", rank = 2)]
pub async fn post_create_coord(db: Db, _claims: CoordClaims, new_form: Json<NewFormWithQuestions>) -> Result<Json<FormWithQuestions>, Status> {
    let form = new_form.into_inner();

    let (form, questions) = create_helper(&db, form).await?;

    // Prepare the response
    let form_expanded = FormWithQuestions {
        id: form.id,
        name: form.name,
        description: form.description,
        questions,
    };

    Ok(Json(form_expanded))
}

#[post("/", data = "<new_form>", rank = 3)]
pub async fn post_create_thera(db: Db, _claims: CoordClaims, new_form: Json<NewFormWithQuestions>) -> Result<Json<FormWithQuestions>, Status> {
    let form = new_form.into_inner();

    let (form, questions) = create_helper(&db, form).await?;

    // Prepare the response
    let form_expanded = FormWithQuestions {
        id: form.id,
        name: form.name,
        description: form.description,
        questions,
    };

    Ok(Json(form_expanded))
}

#[post("/", data = "<_form>", rank = 5)]
pub async fn post_create_none(_form: Json<NewFormWithQuestions>) -> Status {
    println!("Module: form - Controller: post_create_none");
    Status::Unauthorized
}

async fn update_helper(db: &Db, id: i32, form: NewFormWithQuestions) -> Result<(Form, Vec<Question>), Status> {
    // Get the questions
    let questions = match form.questions_id.clone() {
        Some(questions) => questions,
        None => vec![],
    };

    // Create the form
    let new_form: NewForm = form.into();
    let form = form_repository::update(&db, id, new_form).await;
    if let Err(e) = form {
        println!("Module: form - Controller: update_helper");
        println!("Error: {:?}", e);
        return Err(Status::InternalServerError);
    }
    let form = form.unwrap();

    // Add the questions to the form
    let questions = form_question_repository::update_questions_in_form(&db, form.id, questions).await;
    if let Err(e) = questions {
        println!("Module: form - Controller: update_helper");
        println!("Error: {:?}", e);
        return Err(Status::InternalServerError);
    }
    let questions = questions.unwrap();

    return Ok((form, questions));
}

#[put("/<id>", data = "<form>", rank = 1)]
pub async fn put_update_admin(
    db: Db,
    _claims: AdminClaims,
    id: i32,
    form: Json<NewFormWithQuestions>,
) -> Result<Json<FormWithQuestions>, Status> {
    let form = form.into_inner();

    let (form, questions) = update_helper(&db, id, form).await?;

    // Prepare the response
    let form_expanded = FormWithQuestions {
        id: form.id,
        name: form.name,
        description: form.description,
        questions,
    };

    Ok(Json(form_expanded))
}

#[put("/<id>", data = "<form>", rank = 2)]
pub async fn put_update_coord(
    db: Db,
    _claims: CoordClaims,
    id: i32,
    form: Json<NewFormWithQuestions>,
) -> Result<Json<FormWithQuestions>, Status> {
    let form = form.into_inner();

    let (form, questions) = update_helper(&db, id, form).await?;

    // Prepare the response
    let form_expanded = FormWithQuestions {
        id: form.id,
        name: form.name,
        description: form.description,
        questions,
    };

    Ok(Json(form_expanded))
}

#[put("/<id>", data = "<form>", rank = 3)]
pub async fn put_update_thera(
    db: Db,
    _claims: TheraClaims,
    id: i32,
    form: Json<NewFormWithQuestions>,
) -> Result<Json<FormWithQuestions>, Status> {
    let form = form.into_inner();

    let (form, questions) = update_helper(&db, id, form).await?;

    // Prepare the response
    let form_expanded = FormWithQuestions {
        id: form.id,
        name: form.name,
        description: form.description,
        questions,
    };

    Ok(Json(form_expanded))
}

#[put("/<_id>", data = "<_form>", rank = 5)]
pub async fn put_update_none(_id: i32, _form: Json<NewForm>) -> Status {
    println!("Module: form - Controller: put_update_none");
    Status::Unauthorized
}
