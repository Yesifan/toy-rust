#[macro_use] extern crate rocket;
mod lark;
mod response;

use rocket::serde::{Serialize, json::Json};
use response::Response;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/login")]
async fn login() -> String {
    let result = lark::oauth();
    if let Ok(val) = result.await{
        val
    }else{
        String::from("Err")
    }
}

#[derive(Serialize)]
struct Message {
    id: u8,
    message: String
}

#[get("/json")]
async fn json() -> Response<Message> {
    Ok(Json(Message {
        id: 1,
        message: String::from("message"),
    }))
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/api", routes![index, login, json])
}
