use mongodb::bson::Document;
use mongodb::Collection;
use mongodb::error::Error;
use rocket::futures::TryStreamExt;
use crate::api::recettes::components::mongo_component::ClientMongoComponent;
use crate::api::recettes::entities::recette_dbo::RecetteDbo;
use crate::core::recettes::entities::recette::Recette;
use crate::core::recettes::services::recettes_repository::RecettesRepository;

pub struct RecetteRepositoryMongo {
    pub collection: Collection<Document>
}

impl ClientMongoComponent for RecetteRepositoryMongo {}

#[async_trait]
impl RecettesRepository for RecetteRepositoryMongo {
    async fn fetch_many(&self) -> Vec<Recette> {
        match self.find_all().await {
            Ok(recettes) => recettes,
            _ => {
                eprintln!("une erreur est survenue lors de la lecture");
                vec![]
            }
        }
    }
}

impl RecetteRepositoryMongo {
    pub async fn new() -> Result<Self, mongodb::error::Error> {
        Ok(
            Self {
                collection: {
                    Self::collection_recettes().await?
                }
            }
        )
    }

    async fn find_all(&self) -> Result<Vec<Recette>, Error> {
        Ok(
            self.collection
                .find(None, None)
                .await?
                .try_collect::<Vec<Document>>()
                .await?
                .into_iter()
                .map(|doc| doc.into())
                .map(|dbo: RecetteDbo| dbo.into())
                .collect::<Vec<Recette>>()
                .into()
        )
    }
}