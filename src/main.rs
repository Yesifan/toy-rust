#[macro_use] extern crate rocket;
mod lark;

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

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/api", routes![index, login])
}
