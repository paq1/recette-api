use rocket::{Build, Rocket};
use crate::api::recettes::services::recettes_repository_mongo::RecetteRepositoryMongo;
use crate::api::recettes::routes::recettes_read_router::get_recettes;
use crate::api::recettes::routes::recettes_write_router::create_command;
use crate::core::recettes::errors::custom::CustomError;

pub struct AppLauncher;

impl AppLauncher {
    pub async fn launch_rocket() -> Result<Rocket<Build>, CustomError> {
        RecetteRepositoryMongo::new().await
            .map(|recettes_repository| {
                rocket::build()
                    .manage(recettes_repository)
                    .mount(
                        "/",
                        routes![
                            get_recettes,
                            create_command
                        ]
                    )
            })
            .map_err(|err| CustomError::new(err.to_string().as_str()))
    }
}
