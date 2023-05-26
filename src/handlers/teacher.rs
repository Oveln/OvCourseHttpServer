use actix_web::{HttpResponse, web};

use crate::{state::AppState, errors::MyError, db_access::{teacher::*}, models::teacher::{CreateTeacher, UpdateTeacher}};

pub async fn get_all_teacher(
    app_state: web::Data<AppState>,
)-> Result<HttpResponse, MyError> {
    get_all_teacher_db(&app_state.db).await
    .map(|teachers| HttpResponse::Ok().json(teachers))
}

pub async fn get_teacher_details(
    app_state: web::Data<AppState>,
    parmas: web::Path<i32>
)-> Result<HttpResponse, MyError> {
    let teacher_id = parmas.into_inner();
    get_teacher_details_db(&app_state.db, teacher_id).await
    .map(|teacher| HttpResponse::Ok().json(teacher))
}

pub async fn post_new_teacher(
    new_teacher: web::Json<CreateTeacher>,
    app_state: web::Data<AppState>,
)-> Result<HttpResponse, MyError> {
    post_new_teacher_db(&app_state.db, new_teacher.into()).await
    .map(|teacher| HttpResponse::Ok().json(teacher))
}

pub async fn delete_teacher(
    app_state: web::Data<AppState>,
    parmas: web::Path<i32>
)-> Result<HttpResponse, MyError> {
    let teacher_id = parmas.into_inner();
    delete_teacher_db(&app_state.db, teacher_id).await
    .map(|message| HttpResponse::Ok().json(message))
}
pub async fn update_teacher_details(
    update_teacher: web::Json<UpdateTeacher>,
    app_state: web::Data<AppState>,
    parmas: web::Path<i32>
)-> Result<HttpResponse, MyError> {
    let teacher_id = parmas.into_inner();
    update_teacher_details_db(&app_state.db, teacher_id, update_teacher.into())
    .await
    .map(|teacher| HttpResponse::Ok().json(teacher))
}