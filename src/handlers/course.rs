use crate::{models::course::{CreateCourse, UpdateCourse}, state::AppState, errors::MyError, db_access::course::*};
use actix_web::{web, HttpResponse};


pub async fn post_new_course(
    new_course: web::Json<CreateCourse>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, MyError> {
    post_new_course_db(&app_state.db, new_course.try_into()?).await
    .map(|course| HttpResponse::Ok().json(course))
}

pub async fn get_courses_for_teacher(
    app_state: web::Data<AppState>,
    parmas: web::Path<i32>
) -> Result<HttpResponse, MyError> {
    let teacher_id = i32::try_from(parmas.into_inner()).unwrap();
    get_courses_for_teacher_db(&app_state.db, teacher_id).await
    .map(|course|HttpResponse::Ok().json(course))
}

pub async fn get_course_detail(
    app_state: web::Data<AppState>,
    parmas: web::Path<(i32, i32)>
) -> Result<HttpResponse, MyError> {
    // let x = parmas.into_inner();
    // let teacher_id = i32::try_from(x.0).unwrap();
    // let course_id = i32::try_from(x.1).unwrap();
    let (teacher_id, course_id) = parmas.into_inner();
    get_course_detail_db(&app_state.db, teacher_id, course_id).await
    .map(|course| HttpResponse::Ok().json(course))
}

pub async fn update_course_details(
    app_state: web::Data<AppState>,
    update_course: web::Json<UpdateCourse>,
    parmas: web::Path<(i32, i32)>
) -> Result<HttpResponse, MyError> {
    let (teacher_id, course_id) = parmas.into_inner();
    update_course_details_db(&app_state.db, teacher_id, course_id, update_course.into()).await
    .map(|course| HttpResponse::Ok().json(course))
}

pub async fn delete_course(
    app_state: web::Data<AppState>,
    parmas: web::Path<(i32, i32)>
) -> Result<HttpResponse, MyError> {
    let (teacher_id, course_id) = parmas.into_inner();
    delete_course_db(&app_state.db, teacher_id, course_id).await
    .map(|course| HttpResponse::Ok().json(course))
}