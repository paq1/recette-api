use crate::core::recettes::entities::recette::Recette;

#[async_trait]
pub trait RecettesRepository<Response> {
    async fn fetch_many(&self) -> Vec<Recette>;
    async fn insert_recette(&self, recette: Recette) -> Response;
}