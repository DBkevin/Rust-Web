use crate::errors::MyError;
use crate::models::teacher::{CreateTeacher, Teacher, UpdateTeacher};
use sqlx::mysql::MySqlPool;
pub async fn get_all_teachers_db(pool: &MySqlPool) -> Result<Vec<Teacher>, MyError> {
    let rows = sqlx::query!("SELECT id,name,picture_url,profile FROM teacher")
        .fetch_all(pool)
        .await?;
    let teachers: Vec<Teacher> = rows
        .iter()
        .map(|r| Teacher {
            id: r.id as i32,
            name: r.name.clone(),
            picture_url: r.picture_url.clone(),
            profile: r.profile.clone(),
        })
        .collect();
    match teachers.len() {
        0 => Err(MyError::NotFound("No teachers found".into())),
        _ => Ok(teachers),
    }
}
pub async fn get_teacher_details_db(pool: &MySqlPool, teacher_id: i32) -> Result<Teacher, MyError> {
    let row = sqlx::query!(
        "SELECT id,name,picture_url,profile FROM teacher where id=?",
        teacher_id
    )
    .fetch_optional(pool)
    .await?;
    if let Some(teacher) = row {
        Ok(Teacher {
            id: teacher.id as i32,
            name: teacher.name.clone(),
            picture_url: teacher.picture_url.clone(),
            profile: teacher.profile.clone(),
        })
    } else {
        Err(MyError::NotFound("Teacher id  not found".into()))
    }
}

pub async fn post_new_teacher_db(
    pool: &MySqlPool,
    new_teacher: CreateTeacher,
) -> Result<Teacher, MyError> {
    let row = sqlx::query!(
        "INSERT INTO teacher (name,picture_url,profile) VALUES(?,?,?)",
        new_teacher.name,
        new_teacher.picture_url,
        new_teacher.profile
    )
    .execute(pool)
    .await?
    .last_insert_id();
    Ok(Teacher {
        id: row as i32,
        name: new_teacher.name.clone(),
        picture_url: new_teacher.picture_url.clone(),
        profile: new_teacher.profile.clone(),
    })
}

pub async fn update_teacher_details_db(
    pool: &MySqlPool,
    teacher_id: i32,
    update_teacher: UpdateTeacher,
) -> Result<Teacher, MyError> {
    let row = sqlx::query!(
        "select id,name,picture_url,profile from teacher where id=?",
        teacher_id
    )
    .fetch_one(pool)
    .await
    .map_err(|_err| MyError::NotFound("Teacher id not found".into()))?;
    let temp = Teacher {
        id: row.id as i32,
        name: if let Some(name) = update_teacher.name {
            name
        } else {
            row.name.clone()
        },
        picture_url: if let Some(picture_url) = update_teacher.picture_url {
            picture_url
        } else {
            row.picture_url.clone()
        },
        profile: if let Some(profile) = update_teacher.profile {
            profile
        } else {
            row.profile.clone()
        },
    };
    let up_row = sqlx::query!(
        "UPDATE teacher Set name=?,picture_url=?,profile=? where id=?",
        temp.name,
        temp.picture_url,
        temp.profile,
        temp.id
    )
    .execute(pool)
    .await?
    .rows_affected();
    //更新成功会返回更新的条数，如果rows_affeced() 能获取到1，说明成功，否则就失败了
    match up_row {
        1 => Ok(temp),
        _ => Err(MyError::NotFound("Teacher is not update".into())),
    }
}

pub async fn delete_teacher_db(pool: &MySqlPool, teacher_id:i32) -> Result<String, MyError> {
    let row = sqlx::query!("DELETE FROM teacher where id=?", teacher_id)
        .execute(pool)
        .await?
        .rows_affected();
    match row {
        1 => Ok(format!("成功删除{}条记录", row)),
        _ => Err(MyError::DBError("Delete Fail".into())),
    }
    //todo!()
}
