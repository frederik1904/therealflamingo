#![feature(decl_macro)]
#[macro_use] extern crate rocket;
use rocket::form::Form;
mod pdf_generators;

#[derive(Responder)]
#[response(status = 200, content_type = "pdf")]
struct Pdf(Vec<u8>);
 
#[derive(FromForm)]
struct Names {
    name: String
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[post("/pdf", data="<name>")]
async fn pdf(name: Form<Names>) -> Pdf {
    Pdf(pdf_generators::cross_list_generator::generate_cross_list(name.name.clone()))
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, pdf])
}
