use mongodb::bson::Document;
use mongodb::Collection;
use mongodb::error::Error;
use mongodb::results::InsertOneResult;
use rocket::futures::TryStreamExt;
use crate::rocket::futures::TryFutureExt;
use crate::api::recettes::components::mongo_component::ClientMongoComponent;
use crate::api::recettes::entities::recette_dbo::RecetteDbo;
use crate::core::recettes::entities::recette::Recette;
use crate::core::recettes::errors::custom::CustomError;
use crate::core::recettes::services::recettes_repository::RecettesRepository;

pub struct RecetteRepositoryMongo {
    pub collection: Collection<Document>
}

impl ClientMongoComponent for RecetteRepositoryMongo {}

#[async_trait]
impl RecettesRepository<Result<InsertOneResult, CustomError>> for RecetteRepositoryMongo {
    async fn fetch_many(&self) -> Vec<Recette> {
        match self.find_all().await {
            Ok(recettes) => recettes,
            _ => {
                eprintln!("une erreur est survenue lors de la lecture");
                vec![]
            }
        }
    }

    async fn insert_recette(&self, recette: Recette) -> Result<InsertOneResult, CustomError> {
        self.insert_recette_without_check(&recette).await
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

    async fn insert_recette_without_check(&self, recette: &Recette) -> Result<InsertOneResult, CustomError> {
        let document: Document = recette.clone().into();
        self.collection
            .insert_one(document, None)
            // .await
            .map_err(|_| CustomError::new("erreur lors de l'insertion en base"))
            .await
    }
}