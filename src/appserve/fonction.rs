use actix_web::web;
use actix_web::{get, post};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use validator::Validate;
use sqlx::PgPool;
use crate::appserve::erreur::{PersonError, ErrorMessage};
use crate::appserve::erreur::ResponseErrorTrait;


#[derive(Debug, Validate, Deserialize, Serialize, PartialEq, Eq, Clone, FromRow)]
pub struct Unit {
    pub uuid: String,
    #[validate(length(min = 1, message = "name is required"))]
    pub name: String,
    #[validate(length(min = 1, message = "title is required"))]
    pub title: String,
    #[validate(length(min = 1, message = "title is required"))]
    pub categories :  String ,
}    



pub struct Database {
    pub pool: PgPool,
}

#[post("/api")]
pub async fn add(
    unit: web::Json<Unit>,
    pool: web::Data<Database>,
) -> Result<web::Json<Unit>, actix_web::Error> {

    // Validate the incoming data
    if let Err(_validation_error) = unit.validate() {
        let error_message = ErrorMessage::create(PersonError::PersonCreationFailure);
        return Err(actix_web::error::ErrorBadRequest(error_message));
    }

    let query = r#"
        INSERT INTO units (uuid, name, title, categories)
        VALUES ($1, $2, $3, $4)
        RETURNING uuid, name, title, categories
    "#;

    // Attempt to execute the query
    let result = sqlx::query_as::<_, Unit>(query)
        .bind(&unit.uuid)
        .bind(&unit.name)
        .bind(&unit.title)
        .bind(&unit.categories)
        .fetch_one(&pool.pool)
        .await;

    // Match on the result to handle potential errors
    match result {
        Ok(created_unit) => Ok(web::Json(created_unit)),
        Err(sqlx::Error::RowNotFound) => {
            let error_message = ErrorMessage::create(PersonError::PersonNotFound);
            Err(actix_web::error::ErrorNotFound(error_message))
        }
        Err(_) => {
            let error_message = ErrorMessage::create(PersonError::PersonCreationFailure);
            Err(actix_web::error::ErrorInternalServerError(error_message))
        }
    }
}
