use rocket::http::Status;
use rocket::serde::json::Json;

use crate::app::providers::guards::admin::AdminClaims;
use crate::app::providers::guards::coord::CoordClaims;
use crate::app::providers::guards::thera::TheraClaims;
use crate::app::providers::guards::user::UserClaims;
use crate::config::database::Db;

use crate::app::modules::question::model::{NewQuestion, Question};
use crate::app::modules::question::services::repository as question_repository;

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
    ]
}

#[get("/", rank = 4)]
pub async fn get_index_user(db: Db, _claims: UserClaims) -> Result<Json<Vec<Question>>, Status> {
    let questions = question_repository::get_all(&db).await;

    match questions {
        Ok(questions) => Ok(Json(questions)),
        Err(e) => {
            println!("Module: question - Controller: get_index_user");
            println!("Error: {:?}", e);
            Err(Status::InternalServerError)
        }
    }
}

#[get("/", rank = 5)]
pub async fn get_index_none() -> Status {
    println!("Module: question - Controller: get_index_none");
    Status::Unauthorized
}

#[get("/<id>", rank = 4)]
pub async fn get_show_user(db: Db, _claims: UserClaims, id: i32) -> Result<Json<Question>, Status> {
    let question = question_repository::get_by_id(&db, id).await;

    match question {
        Ok(question) => Ok(Json(question)),
        Err(e) => {
            println!("Module: question - Controller: get_show_user");
            println!("Error: {:?}", e);
            Err(Status::InternalServerError)
        }
    }
}

#[get("/<_id>", rank = 5)]
pub async fn get_show_none(_id: i32) -> Status {
    println!("Module: question - Controller: get_show_none");
    Status::Unauthorized
}

#[post("/", data = "<question>", rank = 1)]
pub async fn post_create_admin(
    db: Db,
    _claims: AdminClaims,
    question: Json<NewQuestion>,
) -> Result<Json<Question>, Status> {
    let question = question_repository::add(&db, question.into_inner()).await;

    match question {
        Ok(question) => Ok(Json(question)),
        Err(e) => {
            println!("Module: question - Controller: post_create_admin");
            println!("Error: {:?}", e);
            Err(Status::InternalServerError)
        }
    }
}

#[post("/", data = "<question>", rank = 2)]
pub async fn post_create_coord(
    db: Db,
    _claims: CoordClaims,
    question: Json<NewQuestion>,
) -> Result<Json<Question>, Status> {
    let question = question_repository::add(&db, question.into_inner()).await;

    match question {
        Ok(question) => Ok(Json(question)),
        Err(e) => {
            println!("Module: question - Controller: post_create_coord");
            println!("Error: {:?}", e);
            Err(Status::InternalServerError)
        }
    }
}

#[post("/", data = "<question>", rank = 3)]
pub async fn post_create_thera(
    db: Db,
    _claims: TheraClaims,
    question: Json<NewQuestion>,
) -> Result<Json<Question>, Status> {
    let question = question_repository::add(&db, question.into_inner()).await;

    match question {
        Ok(question) => Ok(Json(question)),
        Err(e) => {
            println!("Module: question - Controller: post_create_thera");
            println!("Error: {:?}", e);
            Err(Status::InternalServerError)
        }
    }
}

#[post("/", data = "<_question>", rank = 5)]
pub async fn post_create_none(_question: Json<NewQuestion>) -> Status {
    println!("Module: question - Controller: post_create_none");
    Status::Unauthorized
}

#[put("/<id>", data = "<question>", rank = 1)]
pub async fn put_update_admin(db: Db, _claims: AdminClaims, id: i32, question: Json<NewQuestion>) -> Result<Json<Question>, Status> {
    let question = question_repository::update(&db, id, question.into_inner()).await;

    match question {
        Ok(question) => Ok(Json(question)),
        Err(e) => {
            println!("Module: question - Controller: put_update_admin");
            println!("Error: {:?}", e);
            Err(Status::InternalServerError)
        }
    }
}

#[put("/<id>", data = "<question>", rank = 2)]
pub async fn put_update_coord(db: Db, _claims: CoordClaims, id: i32, question: Json<NewQuestion>) -> Result<Json<Question>, Status> {
    let question = question_repository::update(&db, id, question.into_inner()).await;

    match question {
        Ok(question) => Ok(Json(question)),
        Err(e) => {
            println!("Module: question - Controller: put_update_coord");
            println!("Error: {:?}", e);
            Err(Status::InternalServerError)
        }
    }
}

#[put("/<id>", data = "<question>", rank = 3)]
pub async fn put_update_thera(db: Db, _claims: TheraClaims, id: i32, question: Json<NewQuestion>) -> Result<Json<Question>, Status> {
    let question = question_repository::update(&db, id, question.into_inner()).await;

    match question {
        Ok(question) => Ok(Json(question)),
        Err(e) => {
            println!("Module: question - Controller: put_update_thera");
            println!("Error: {:?}", e);
            Err(Status::InternalServerError)
        }
    }
}

#[put("/<_id>", data = "<_question>", rank = 5)]
pub async fn put_update_none(_id: i32, _question: Json<NewQuestion>) -> Status {
    println!("Module: question - Controller: put_update_none");
    Status::Unauthorized
}
