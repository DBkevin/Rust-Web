use chrono::DateTime;
use chrono::NaiveDateTime;
use chrono::Utc;
use dotenv::dotenv;
use sqlx::mysql::MySql;
use sqlx::mysql::MySqlPoolOptions;
use std::env;
use std::io;
#[derive(Debug)]
pub struct  Course{
	pub id:u32,
	pub teacher_id:u32,
	pub name:String,
	pub time:Option<DateTime<Utc>>,
}
#[actix_web::main]
async fn main()->io::Result<()>{
	dotenv().ok();
	let database_url=env::var("DATABASE_URL").expect("DaTABASE_URL没有设置");
	let dbpool=MySqlPoolOptions::new().connect(&database_url).await.unwrap();
	let course_rows=sqlx::query!(
		r#"select id, teacher_id,name,time from course where id=? "#,1
	)
	.fetch_all(&dbpool)
	.await
	.unwrap();
	let mut  courses_list=vec![];
	for row in course_rows{
		courses_list.push(Course{
			id:row.id,
			teacher_id:row.teacher_id,
			name:row.name,
			time:Some(row.time),
		})
	}
	print!("Courses={:?}",courses_list);
	Ok(())

}