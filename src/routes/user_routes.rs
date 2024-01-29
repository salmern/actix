use std::process::id;
// use crate::telemetry;
use crate::ApiResponse;
use crate::AppState;
use crate::UserProfileTbl;
use actix_web::delete;
use actix_web::put;
use actix_web::{get, post, web, Error, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use tracing::{info, error, span, Level};
use tracing_futures::Instrument;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NewUserPayload {
    pub name: String,
    pub email: String,
    pub role: String,
    pub mobile_number: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UpdateUserPayload {
    pub name: Option<String>,
    pub role: Option<String>,
    pub mobile_number: Option<String>,
}

// #[post("user/create")]

pub async fn add_new_user(
    payload: web::Json<NewUserPayload>,
    state: web::Data<AppState>,
) -> impl Responder {
    // HttpResponse::Ok().finish()

    // let span = telemetry_span!(tracing::Level::INFO, "add_new_user");
    // telemetry_enter!(span);

    let span = span!(Level::INFO, "add_new_user");
    let _enter = span.enter();
    let user = UserProfileTbl::add_user(&state.db_pool, payload.0)
    .instrument(span.clone())
    .await;


    match user {
        
        Ok(x) =>{
            // telemetry::log_info("Successfully added a new user");
            info!("Successfully added a new user");
            HttpResponse::Ok().json(serde_json::json!(x))
        }
        Err(e) =>{ 
            // telemetry::log_error(&format!("Error adding new user: {}", e));
            HttpResponse::InternalServerError().body(e.to_string())

        }

    }
}

// #[get("/user/{email}")]
// pub async fn get_user_by_email(
//     web::Path(email): web::Path<String>,
//     state: web::Data<AppState>,
// ) -> impl Responder {
//     match UserProfileTbl::get_user_by_email(&state.db_pool, &email).await {
//         Ok(Some(user)) => HttpResponse::Ok().json(user),
//         Ok(None) => HttpResponse::NotFound().body("User not found"),
//         Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
//     }
// }
#[get("/user/email/{email}")]
pub async fn get_user_by_email(
    email: web::Path<String>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    match UserProfileTbl::get_user_by_email(&state.db_pool, &email).await {
        // Ok(Some(user)) => HttpResponse::Ok().json(user),
        Ok(Some(user)) => {
            let response = ApiResponse {
                success: true,
                message: "User found".to_string(),
                data: Some(user),
            };
            Ok(HttpResponse::Ok().json(response))
        }
        // Ok(None) => HttpResponse::NotFound().body("User not found"),
        Ok(None) => {
            let response = ApiResponse {
                success: false,
                message: "User not found".to_string(),
                data: None,
            };
            Ok(HttpResponse::NotFound().json(response))
        }
        // Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
        Err(e) => {
            let response = ApiResponse {
                success: false,
                message: e.to_string(),
                data: None,
            };
            Ok(HttpResponse::InternalServerError().json(response))
        }
    }
}

#[get("/user/id/{id}")]
pub async fn get_user_by_id(
    id: web::Path<uuid::Uuid>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    // let r = UserProfileTbl::get_user_by_id(&state.db_pool, id.into_inner()).await;

    // match r {
    //     Ok(Some(user)) => Ok(HttpResponse::Ok().json(user)),
    //     Ok(None) => Ok(HttpResponse::NotFound().finish()),
    //     Err(e) => Ok(HttpResponse::BadRequest().json(e.to_string())),
    // }
    

    match UserProfileTbl::get_user_by_id(&state.db_pool, id.into_inner()).await {
        // Ok(Some(user)) => HttpResponse::Ok().json(user),
        Ok(Some(user)) => {
            let response = ApiResponse {
                success: true,
                message: "User found".to_string(),
                data: Some(user),
            };
             Ok(HttpResponse::Ok().json(response))
        }
        // Ok(None) => HttpResponse::NotFound().body("User not found"),
        Ok(None) => {
            let response = ApiResponse {
                success: false,
                message: "User not found".to_string(),
                data: None,
            };
            Ok(HttpResponse::NotFound().json(response))
        }
        // Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
        Err(e) => {
            let response = ApiResponse {
                success: false,
                message: e.to_string(),
                data: None,
            };
            Ok(HttpResponse::InternalServerError().json(response))
        }
    }
}

#[put("/user/update/{uuid}")]
pub async fn update_user(
    path: web::Path<uuid::Uuid>,
    payload: web::Json<UpdateUserPayload>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, actix_web::Error> {
    let updated_user =
        UserProfileTbl::update_user(path.into_inner(), payload.0, &state.db_pool).await;

    match updated_user {
        Ok(Some(user)) => {
            // User was successfully updated
            Ok(HttpResponse::Ok().json(user))
        }
        Ok(None) => {
            // User not found
            Err(actix_web::error::ErrorNotFound("User not found"))
        }
        Err(e) => {
            // Internal server error
            eprintln!("Error updating user: {:?}", e);
            Err(actix_web::error::ErrorInternalServerError(
                "Internal Server Error",
            ))
        }
    }
}

//Delete user by id 
#[delete("/user/delete_id/{id}")]
pub async fn delete_user_by_id(
    id: web::Path<uuid::Uuid>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    match UserProfileTbl::delete_user_by_id(&state.db_pool, id.into_inner()).await {
        Ok(true) => {
            let response = ApiResponse {
                success: true,
                message: "User deleted successfully".to_string(),
                data: None,
            };
            Ok(HttpResponse::Ok().json(response))
        }
        Ok(false) => {
            let response = ApiResponse {
                success: false,
                message: "User not found".to_string(),
                data: None,
            };
            Ok(HttpResponse::NotFound().json(response))
        }
        Err(e) => {
            let response = ApiResponse {
                success: false,
                message: e.to_string(),
                data: None,
            };
            Ok(HttpResponse::InternalServerError().json(response))
        }
    }
}

