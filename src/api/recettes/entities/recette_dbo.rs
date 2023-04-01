use mongodb::bson::{Bson, Document};
use mongodb::bson::oid::ObjectId;
use rocket::serde::{Deserialize, Serialize};

use crate::core::recettes::entities::recette::Recette;

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct RecetteDbo {
    pub _id: ObjectId,
    pub name: String,
    pub ingredients: Vec<String>,
    pub etapes: Vec<String>,
}

impl From<RecetteDbo> for Recette {
    fn from(value: RecetteDbo) -> Self {
        Recette {
            name: value.name,
            ingredients: value.ingredients.clone(),
            etapes: value.etapes.clone(),
        }
    }
}

impl From<Document> for RecetteDbo {
    fn from(value: Document) -> Self {
        mongodb::bson::from_bson(Bson::Document(value))
            .unwrap()
    }
}
