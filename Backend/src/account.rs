

use rocket::response::content;

#[get("/bar")]
pub fn foo() -> content::Json<&'static str> {
    content::Json("{\"text\": \"Hello World 4\"}")
}