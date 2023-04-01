use crate::models::recettes::views::recette_view::RecetteView;

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