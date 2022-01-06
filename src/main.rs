#![feature(decl_macro)]

mod Book;

#[macro_use] extern crate rocket;

use rocket::Request;
use rocket::response::content::Json;
use rocket::request::Form;
use rocket_contrib::templates::Template;
use serde::Serialize;

#[derive(FromForm,Debug)]
struct Book{
    title: String,
    author: String,
    genre:String
}


#[catch(404)]
fn not_found(req: &Request) -> String {
    format!("We couldn't find the request path '{}'", req.uri())
}
#[get("/")]
fn index() -> Template {
    #[derive(Serialize)]
    struct Context {
        first_name: String,
        last_name: String
    }

    let context = Context{
        first_name:String::from("Gold"),
        last_name:String::from("Silver")
    };
    Template::render("home",context)
}

#[post("/book", data="<book_form>")]
fn new_book(book_form:Form<Book>) -> String {
    let book:Book = book_form.into_inner(); // get the req body
    let mut db : Vec<Book> = Vec::new();
    db.push(book);
    format!("Book added succesfully: {:?}",db)

}
fn main() {
    rocket::ignite()
        .register(catchers![not_found])
        .mount("/",routes![index])
        .mount("/api", routes![new_book])
        .attach(Template::fairing())
        .launch();
}
