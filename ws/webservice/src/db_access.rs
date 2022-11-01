use super::models::*;
use sqlx::{mysql::MySqlPool};

pub async fn get_courses_for_teacher_db(pool: &MySqlPool, teacher_id: i32) -> Vec<Course> {
    let rows = sqlx::query!(
        r#"SELECT id, teacher_id,name,time FROM course where teacher_id=?"#,
        teacher_id
    )
    .fetch_all(pool)
    .await
    .unwrap();
    rows.iter()
        .map(|r| Course {
            id: Some(r.id),
            teacher_id: r.teacher_id,
            name: r.name.clone(),
            time: Some(r.time)
        })
        .collect()
}
pub async fn get_couse_details_db(pool: &MySqlPool, teacher_id: i32, course_id: i32) -> Course {
    let row = sqlx::query!(
        r#"SELECT id,teacher_id,name,time 
			FROM course
			WHERE teacher_id=? and id=?
		"#,
        teacher_id,
        course_id
    )
    .fetch_one(pool)
    .await
    .unwrap();
    Course {
        id: Some(row.id),
        teacher_id: row.teacher_id,
        name: row.name.clone(),
        time: Some(row.time),
    }
}

pub async fn post_new_course_db(pool: &MySqlPool, new_course: Course) -> Course{
    let row = sqlx::query!(
        r#"INSERT INTO course (id,teacher_id,name)
		VALUES(?,?,?)
		"#,
        new_course.id,
        new_course.teacher_id,
        new_course.name
    )
    .fetch_one(pool)
    .await;
    new_course
}
