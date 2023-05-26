use sqlx::PgPool;

use crate::{models::teacher::{Teacher, CreateTeacher, UpdateTeacher}, errors::MyError};

pub async fn get_all_teacher_db(pool: &PgPool)
-> Result<Vec<Teacher>,MyError> {
    sqlx::query_as!(
        Teacher,
        "SELECT * FROM teacher"
    ).fetch_all(pool).await
    .map_err(|_err|MyError::NotFound("Not found teacher".into()))
}

pub async fn get_teacher_details_db(
    pool: &PgPool, 
    teacher_id: i32
) 
-> Result<Teacher, MyError>{
    sqlx::query_as!(
        Teacher,
        "SELECT * FROM teacher
        WHERE teacher.id = $1",
        teacher_id
    ).fetch_one(pool).await
    .map_err(|_err| MyError::NotFound("Teacher Id not found".into()))
}

pub async fn post_new_teacher_db(
    pool: &PgPool,
    new_teacher: CreateTeacher
) -> Result<Teacher,MyError> {
    sqlx::query_as!(
        Teacher,
        "INSERT INTO teacher (name, picture_url, profile)
        VALUES ($1,$2,$3)
        RETURNING id, name, picture_url, profile",
        new_teacher.name,
        new_teacher.picture_url,
        new_teacher.profile
    ).fetch_one(pool).await
    .map_err(|err| MyError::from(err))
}

pub async fn update_teacher_details_db(
    pool: &PgPool,
    teacher_id: i32,
    update_teacher: UpdateTeacher
) -> Result<Teacher, MyError> {
    let row = sqlx::query_as!(
        Teacher,
        "SELECT * FROM teacher WHERE teacher.id = $1",
        teacher_id
    ).fetch_one(pool).await.map_err(|_err| MyError::NotFound("Teacher Id Not found".into()))?;
    let temp = Teacher {
        id: row.id,
        name: if let Some(name) = update_teacher.name {
            name
        } else {
            row.name
        },
        picture_url: if let Some(picture_url) = update_teacher.picture_url {
            picture_url
        } else {
            row.picture_url
        },
        profile: if let Some(profile) = update_teacher.profile {
            profile
        } else {
            row.profile
        },
    };
    sqlx::query_as!(
        Teacher,
        "UPDATE teacher SET name=$1, picture_url=$2, profile=$3
        WHERE teacher.id = $4
        RETURNING id, name, picture_url, profile",
        temp.name, temp.picture_url, temp.profile, temp.id
    ).fetch_one(pool).await
    .map_err(|_err| MyError::NotFound("Teacher Id not found".into()))
}

pub async fn delete_teacher_db(
    pool: &PgPool,
    teacher_id: i32
) -> Result<String, MyError> {
    let row = sqlx::query(
        &format!("DELETE FROM teacher WHERE id = {}",teacher_id)
    ).execute(pool)
    .await
    .map_err(|_err| MyError::DBError("Unable to delete teacher".into()));
    match row {
        Ok(result) => {
            Ok(format!("Deleted {:?} record", result.rows_affected()))
        }
        Err(err) => {
            Err(err)
        }
    }
}