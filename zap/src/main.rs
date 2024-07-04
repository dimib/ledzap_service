#[macro_use] extern crate rocket;

use rocket_cors::{AllowedHeaders, AllowedOrigins};
mod excuse;
mod presets;

#[launch]
fn rocket() -> _ {
//    let logged_in = LoggedIn::new();
//    let where_is = WhereIsMap::new();


    let cors = rocket_cors::CorsOptions {
        allowed_origins: AllowedOrigins::all(),
        allowed_methods: vec![rocket::http::Method::Get, rocket::http::Method::Post].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::all(),
        allow_credentials: true,
        ..Default::default()
    }.to_cors().unwrap();

    rocket::build()
        .mount("/excuse", routes![
            excuse::service::get_excuse_as_json,
            excuse::service::post_excuse_as_json,
        ])
        .attach(cors)
        .mount("/presets", routes![
            presets::service::get_personas,
            presets::service::get_persona,
        ])
        .register("/", catchers![excuse::service::not_unauthorized, excuse::service::unprocessable_entity])
}
