use rocket::http::Status;
use rocket::serde::json::Json;

use crate::app::providers::guards::admin::AdminClaims;
use crate::app::providers::guards::coord::CoordClaims;
use crate::app::providers::guards::thera::TheraClaims;
use crate::app::providers::guards::user::UserClaims;
use crate::config::database::Db;

use crate::app::modules::form::model::{Form, NewForm};
use crate::app::modules::form::services::repostitory as form_repository;

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
pub async fn get_show_user(db: Db, _claims: UserClaims, id: i32) -> Result<Json<Form>, Status> {
    let form = form_repository::get_by_id(&db, id).await;

    match form {
        Ok(form) => Ok(Json(form)),
        Err(e) => {
            println!("Module: form - Controller: get_show_user");
            println!("Error: {:?}", e);
            Err(Status::InternalServerError)
        }
    }
}

#[get("/<_id>", rank = 5)]
pub async fn get_show_none(_id: i32) -> Status {
    println!("Module: form - Controller: get_show_none");
    Status::Unauthorized
}

#[post("/", data = "<form>", rank = 1)]
pub async fn post_create_admin(
    db: Db,
    _claims: AdminClaims,
    form: Json<NewForm>,
) -> Result<Json<Form>, Status> {
    let form = form_repository::create(&db, form.into_inner()).await;

    match form {
        Ok(form) => Ok(Json(form)),
        Err(e) => {
            println!("Module: form - Controller: post_create_admin");
            println!("Error: {:?}", e);
            Err(Status::InternalServerError)
        }
    }
}

#[post("/", data = "<form>", rank = 2)]
pub async fn post_create_coord(
    db: Db,
    _claims: CoordClaims,
    form: Json<NewForm>,
) -> Result<Json<Form>, Status> {
    let form = form_repository::create(&db, form.into_inner()).await;

    match form {
        Ok(form) => Ok(Json(form)),
        Err(e) => {
            println!("Module: form - Controller: post_create_coord");
            println!("Error: {:?}", e);
            Err(Status::InternalServerError)
        }
    }
}

#[post("/", data = "<form>", rank = 3)]
pub async fn post_create_thera(
    db: Db,
    _claims: TheraClaims,
    form: Json<NewForm>,
) -> Result<Json<Form>, Status> {
    let form = form_repository::create(&db, form.into_inner()).await;

    match form {
        Ok(form) => Ok(Json(form)),
        Err(e) => {
            println!("Module: form - Controller: post_create_thera");
            println!("Error: {:?}", e);
            Err(Status::InternalServerError)
        }
    }
}

#[post("/", data = "<_form>", rank = 5)]
pub async fn post_create_none(_form: Json<NewForm>) -> Status {
    println!("Module: form - Controller: post_create_none");
    Status::Unauthorized
}

#[put("/<id>", data = "<form>", rank = 1)]
pub async fn put_update_admin(
    db: Db,
    _claims: AdminClaims,
    id: i32,
    form: Json<NewForm>,
) -> Result<Json<Form>, Status> {
    let form = form_repository::update(&db, id, form.into_inner()).await;

    match form {
        Ok(form) => Ok(Json(form)),
        Err(e) => {
            println!("Module: form - Controller: put_update_admin");
            println!("Error: {:?}", e);
            Err(Status::InternalServerError)
        }
    }
}

#[put("/<id>", data = "<form>", rank = 2)]
pub async fn put_update_coord(
    db: Db,
    _claims: CoordClaims,
    id: i32,
    form: Json<NewForm>,
) -> Result<Json<Form>, Status> {
    let form = form_repository::update(&db, id, form.into_inner()).await;

    match form {
        Ok(form) => Ok(Json(form)),
        Err(e) => {
            println!("Module: form - Controller: put_update_coord");
            println!("Error: {:?}", e);
            Err(Status::InternalServerError)
        }
    }
}

#[put("/<id>", data = "<form>", rank = 3)]
pub async fn put_update_thera(
    db: Db,
    _claims: TheraClaims,
    id: i32,
    form: Json<NewForm>,
) -> Result<Json<Form>, Status> {
    let form = form_repository::update(&db, id, form.into_inner()).await;

    match form {
        Ok(form) => Ok(Json(form)),
        Err(e) => {
            println!("Module: form - Controller: put_update_thera");
            println!("Error: {:?}", e);
            Err(Status::InternalServerError)
        }
    }
}

#[put("/<_id>", data = "<_form>", rank = 5)]
pub async fn put_update_none(_id: i32, _form: Json<NewForm>) -> Status {
    println!("Module: form - Controller: put_update_none");
    Status::Unauthorized
}
