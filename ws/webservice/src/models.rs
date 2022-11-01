use actix_web::web;
use chrono::{ DateTime, Utc};
use serde::{Deserialize,Serialize};
#[derive(Deserialize,Serialize,Debug,Clone)]
pub struct Course{
	pub teacher_id:u32,
	pub id:Option<u32>,
	pub name:String,
	pub time:Option<DateTime<Utc>>,
}
impl From<web::Json<Course>>for Course{
	fn from(course:web::Json<Course>)->Self{
		Course{
			teacher_id:course.teacher_id,
			id:course.id,
			name:course.name.clone(),
			time:course.time,
		}
	}
}