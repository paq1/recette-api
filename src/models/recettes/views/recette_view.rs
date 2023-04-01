use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct RecetteView {
    pub name: String,
    pub ingredients: Vec<String>,
    pub etapes: Vec<String>,
}
