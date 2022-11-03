use crate::errors::MyError;
use crate::models::course::{Course, CreateCourse, UpdateCourse};
use chrono::prelude::*;
use sqlx::mysql::MySqlPool;

pub async fn get_courses_for_teacher_db(
    pool: &MySqlPool,
    teacher_id: i32,
) -> Result<Vec<Course>, MyError> {
    let rows: Vec<Course> = sqlx::query_as!(
        Course,
        r#"SELECT * 
        FROM course where teacher_id=?
        "#,
        teacher_id
    )
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

pub async fn get_couse_details_db(
    pool: &MySqlPool,
    teacher_id: i32,
    course_id: i32,
) -> Result<Course, MyError> {
    let row = sqlx::query_as!(
        Course,
        r#"SELECT *
			FROM course
			WHERE teacher_id=? and id=?
		"#,
        teacher_id,
        course_id
    )
    .fetch_optional(pool)
    .await?;
    if let Some(course) = row {
        Ok(course)
    } else {
        Err(MyError::NotFound("Course ID not found".into()))
    }
}

pub async fn post_new_course_db(
    pool: &MySqlPool,
    new_course: CreateCourse,
) -> Result<Course, MyError> {
    let time = Utc::now();
    let row=sqlx::query_as!(
        Course,
        r#"
        INSERT INTO course (teacher_id,name,time,description,format,structure,duration,price,language,level)
        VALUES(?,?,?,?,?,?,?,?,?,?)
        "#,
        new_course.teacher_id,new_course.name,time,new_course.description,new_course.format,new_course.structure,new_course.duration,new_course.price,new_course.language,new_course.level
     )
    .execute(pool)
    .await?
    .last_insert_id();
    Ok(Course {
        teacher_id: new_course.teacher_id,
        id: row as u32,
        name: new_course.name,
        time,
        description: new_course.description,
        format: new_course.format,
        structure: new_course.structure,
        duration: new_course.duration,
        price: new_course.price,
        language: new_course.language,
        level: new_course.level,
    })
}

pub async fn delete_course_db(
    pool: &MySqlPool,
    teacher_id: i32,
    id: i32,
) -> Result<String, MyError> {
    let course_row = sqlx::query("DELETE FROM course where teacher_id=? and id=?")
        .bind(teacher_id)
        .bind(id)
        .execute(pool)
        .await?
        .rows_affected();
    Ok(format!("Deleted{}record", course_row).to_string())
}

pub async fn update_couse_details_db(
    pool: &MySqlPool,
    teacher_id: i32,
    id: i32,
    update_course: UpdateCourse,
) -> Result<Course, MyError> {
    let current_course_row = sqlx::query_as!(
        Course,
        "SELECT * From course where teacher_id=? and id=?",
        teacher_id,
        id
    )
    .fetch_one(pool)
    .await?;
    let name: String = if let Some(name) = update_course.name {
        name
    } else {
        current_course_row.name
    };
    let description: String = if let Some(description) = update_course.description {
        description
    } else {
        current_course_row.description.unwrap_or_default()
    };
    let format: String = if let Some(format) = update_course.format {
        format
    } else {
        current_course_row.format.unwrap_or_default()
    };
    let structure: String = if let Some(structure) = update_course.structure {
        structure
    } else {
        current_course_row.structure.unwrap_or_default()
    };
    let duration: String = if let Some(duration) = update_course.duration {
        duration
    } else {
        current_course_row.duration.unwrap_or_default()
    };
    let price = if let Some(price) = update_course.price {
        price
    } else {
        current_course_row.price.unwrap_or_default()
    };

    let language = if let Some(language) = update_course.language {
        language
    } else {
        current_course_row.language.unwrap_or_default()
    };

    let time = current_course_row.time;

    let level = if let Some(level) = update_course.level {
        level
    } else {
        current_course_row.level.unwrap_or_default()
    };
    let course_row=sqlx::query_as!(
        Course,
        "UPdate course Set name=?,description=?,format=?,structure=?,duration=?,price=?,language=?,level=? where teacher_id=? and id=?",
        name,description,format,structure,duration,price,language,level,teacher_id,id
    )
    .execute(pool)
    .await;
    match course_row {
        Ok(_) => Ok(Course {
            teacher_id: teacher_id as u32,
            id: id as u32,
            time: time,
            description: Some(description),
            format: Some(format),
            structure: Some(structure),
            duration: Some(duration),
            price: Some(price),
            language: Some(language),
            level: Some(level),
            name,
        }),
        Err(e) => {
            println!("更新报错了,原始错误：{:?}", e);
            Err(MyError::NotFound("".to_string()))
        }
    }

    // if let Ok(course)=course_row{

    // }else{
    //     Err(MyError::NotFound("Course id not  found".into()))
    // }
}
