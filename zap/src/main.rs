#[macro_use] extern crate rocket;

mod excuse;
mod presets;

#[launch]
fn rocket() -> _ {
//    let logged_in = LoggedIn::new();
//    let where_is = WhereIsMap::new();

    rocket::build()
        .mount("/excuse", routes![
            excuse::service::get_excuse_as_json,
            excuse::service::post_excuse_as_json,
        ])
        .mount("/presets", routes![
            presets::service::get_personas,
            presets::service::get_persona,
        ])
        .register("/", catchers![excuse::service::not_unauthorized, excuse::service::unprocessable_entity])
}
