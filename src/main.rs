#[cfg(test)] mod tests;

mod files;

#[macro_use] extern crate rocket;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(files::stage())
}
