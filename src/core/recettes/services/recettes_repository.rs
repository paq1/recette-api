use crate::core::recettes::entities::recette::Recette;

#[async_trait]
pub trait RecettesRepository {
    async fn fetch_many(&self) -> Vec<Recette>;
}