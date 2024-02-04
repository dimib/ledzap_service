
use rocket::serde::json::{Value, json};
use rocket::http::Status;

use crate::presets::model::load_presets;

use super::model::PresetData;

#[get("/", format = "json")]
pub async fn get_personas() -> (Status, Value) {

    let presets = load_presets().or_else(|error| {
        println!("Error loading presets: {}", error);
        Err(Status::InternalServerError)
    });
    match presets {
        Ok(p) => {
            let personas = p.iter().map(|p| p.persona.clone()).collect::<Vec<String>>();
            return (Status::Ok, json!(PresetData {
                status: "OK".to_string(),
                data: personas
            }))
        },
        Err(s) => {
            (s, json!({}))
        }
    }
}

#[get("/<persona>", format = "json")]
pub async fn get_persona(persona: String) -> (Status, Value) {

    let presets = load_presets().or_else(|error| {
        println!("Error loading presets: {}", error);
        Err(Status::InternalServerError)
    });

    match presets {
        Ok(p) => {
            let persona_presets = p.iter().find(|p| p.persona == persona);
            match persona_presets {
                Some(p) => {
                    return (Status::Ok, json!(PresetData {
                        status: "OK".to_string(),
                        data: p.excuses.clone()
                    }))
                },
                None => {
                    return (Status::NotFound, json!(PresetData {
                        status: "ERROR".to_string(),
                        data: vec!["Persona not found".to_string()]
                    }))
                }
            }
        },
        Err(s) => {
            (s, json!({}))
        }
    }
}