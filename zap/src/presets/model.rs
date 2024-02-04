use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;
use std::io::Read;

#[derive(Debug, Deserialize)]
pub struct PresetFile {
    pub presets: Vec<PersonaPreset>,
}

#[derive(Debug, Deserialize)]
pub struct PersonaPreset {
    pub persona: String,
    pub excuses: Vec<String>,
}

pub fn load_presets() -> Result<Vec<PersonaPreset>, Box<dyn Error>> {
    let mut file = File::open("presets.json")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let presets: PresetFile = serde_json::from_str(&contents)?;
    Ok(presets.presets)
}

#[derive(Debug, Serialize)]
pub struct PresetData {
    pub status: String,
    pub data: Vec<String>,
}

