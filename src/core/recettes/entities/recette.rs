use mongodb::bson::{doc, Document};
use crate::models::recettes::commands::create_recette::CreateRecetteCommand;
use crate::models::recettes::views::recette_view::RecetteView;

#[derive(Clone)]
pub struct Recette {
    pub name: String,
    pub ingredients: Vec<String>,
    pub etapes: Vec<String>,
}

impl From<Recette> for RecetteView {
    fn from(value: Recette) -> Self {
        RecetteView {
            name: value.name,
            ingredients: value.ingredients.clone(),
            etapes: value.etapes.clone()
        }
    }
}

impl From<CreateRecetteCommand> for Recette {
    fn from(value: CreateRecetteCommand) -> Self {
        Recette {
            name: value.name,
            ingredients: value.ingredients.clone(),
            etapes: value.etapes.clone()
        }
    }
}

impl From<Recette> for Document {
    fn from(value: Recette) -> Self {
        doc! {
            "name": value.name,
            "ingredients": value.ingredients,
            "etapes": value.etapes
        }
    }
}
