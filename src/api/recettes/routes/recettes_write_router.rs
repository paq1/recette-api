use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::Json;
use rocket::State;
use crate::api::recettes::services::recettes_repository_mongo::RecetteRepositoryMongo;
use crate::core::recettes::services::recettes_repository::RecettesRepository;
use crate::models::recettes::commands::create_recette::CreateRecetteCommand;
use crate::models::recettes::views::json_data_response::JsonDataResponse;

#[post("/recettes/commands/create", data="<create_command>")]
pub async fn create_command(
    recettes_repository: &State<RecetteRepositoryMongo>,
    create_command: Json<CreateRecetteCommand>
) -> Result<Json<JsonDataResponse>, status::Custom<Json<JsonDataResponse>>> {

    let cmd = create_command.0;

    recettes_repository
        .insert_recette(
            cmd.into()
        )
        .await
        .map(|_| {
            Json(
                JsonDataResponse {
                    message: String::from("inserted")
                }
            )
        })
        .map_err(|err| {
            status::Custom(
                Status::BadRequest,
                Json(
                    JsonDataResponse {
                        message: err.message
                    }
                )
            )
        })
}
