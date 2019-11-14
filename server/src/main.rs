// #![feature(proc_macro_hygiene, decl_macro)]

// #[macro_use]
// extern crate rocket;

// use rocket_contrib::serve::StaticFiles;

// fn main() {
//     rocket::ignite()
//         .mount(
//             "/",
//             StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/../client/build")),
//         )
//         .launch();
// }

#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate rocket_contrib;

use rocket_contrib::serve::StaticFiles;

#[get("/hello")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    rocket::ignite()
        .mount(
            "/",
            StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/../client/build")),
        )
        .mount("/", routes![index])
        .launch();
}
