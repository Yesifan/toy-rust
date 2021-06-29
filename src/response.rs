use rocket::serde::{Serialize, json::Json};

#[derive(Serialize)]
pub struct Error {
  code: u32,
  message:String,
}

pub type Response<T> = Result<Json<T>, Json<Error>>;