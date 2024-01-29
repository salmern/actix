mod routes;
// #[macro_use]
// mod telemetry;
mod core;
mod test;
pub use routes::*;
pub use core::*;
use sqlx::{PgPool, postgres::PgPoolOptions, Postgres, Pool};
use dotenv::dotenv;
use tracing_actix_web::TracingLogger;

use tracing_subscriber::FmtSubscriber;



use actix_web::{web::{self, Data}, App, HttpServer, HttpResponse, Responder, Result, HttpRequest, middleware::Compat};
use serde::{Deserialize, Serialize};
use crate::routes::add_new_user;

#[derive(Debug, Serialize, Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct LoginResponse {
    success: bool,
    message: String,
}
#[derive(Debug, Serialize)]
struct ApiResponse {
    success: bool,
    message: String,
    data: Option<UserProfileTbl>,
}
pub struct AppState{
    db_pool: Pool<Postgres>,
}


async fn login(info: web::Json<LoginRequest>) -> Result<impl Responder> {
  
    if info.username == "admin" && info.password == "admin" {
        log::info!("Login successful for user: {}", info.username);
        Ok(HttpResponse::Ok().json(LoginResponse {
            success: true,
            message: "Login successful".to_string(),
        }))
    } else {
         
         log::warn!("Failed login attempt for user: {}", info.username);
        Ok(HttpResponse::Unauthorized().json(LoginResponse {
            success: false,
            message: "Invalid username or password".to_string(),
        }))
    }
}

async fn health_check(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().finish()
    }

    fn setup_tracing() {
        let fmt_subscriber = FmtSubscriber::builder()
            .with_max_level(tracing::Level::TRACE)
            .finish();
    
        tracing::subscriber::set_global_default(fmt_subscriber)
            .expect("setting default subscriber failed");
    }
    

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    setup_tracing();
    // telemetry::init_tracing();
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // let db_pool = PgPool::connect_with(PgConnectOptions::new()
    // .host("localhost")
    // .port(5432)
    // .username("postgres")
    // .password("S@1m@n")
    // .database("pot"))
    // .await
    // .map_err(|err| {
    //     log::error!("Database connection error: {:?}", err);
    //     std::process::exit(1);
    // });
    let pool = PgPoolOptions::new()
    .max_connections(5)
    .connect(&database_url)
    .await
    .expect("Error in building connection pool");

    HttpServer::new(  move|| {
        App::new()
            // .wrap(telemetry::TracingLogger)
            // .wrap(Compat::new(TracingLogger::default()))
            .wrap(TracingLogger::default())
            .service(web::resource("/login")
            .route(web::post().to(login)))
            .service(update_user) 
            .service(get_user_by_email) 
            .service(get_user_by_id) 
            .service(delete_user_by_id) 
            // .service(web::resource("/update_user").route(web::put().to(update_user)))
            .route("/user/create", web::post().to(add_new_user))
            .route("/health_check", web::get().to(health_check))
            // .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(AppState{db_pool:pool.clone()}))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
