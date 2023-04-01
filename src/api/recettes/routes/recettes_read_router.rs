use rocket::serde::json::Json;
use rocket::State;
use crate::api::recettes::services::recettes_repository_mongo::RecetteRepositoryMongo;
use crate::core::recettes::services::recettes_repository::RecettesRepository;

use crate::models::recettes::views::recette_view::RecetteView;

#[get("/recettes")]
pub async fn get_recettes(
    recettes_repository: &State<RecetteRepositoryMongo>
) -> Json<Vec<RecetteView>> {
    Json(
        recettes_repository
            .fetch_many()
            .await
            .into_iter()
            .map(|recette| recette.into())
            .collect::<Vec<_>>()
    )
}