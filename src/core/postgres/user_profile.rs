use chrono::{Local, NaiveDateTime};

use serde::Serialize;
use sqlx::{PgPool, Postgres};
use tracing::{info, error, span, Level};
use crate::{NewUserPayload, UpdateUserPayload};
use sqlx::types::Json;

// enum EncodableValue {
//     Text(String),
//     Json(Json<String>),
//     NaiveDateTime(NaiveDateTime),

// }

#[derive(Debug, Clone, Serialize)]
pub struct UserProfileTbl {
    pub user_id: uuid::Uuid,
    pub name: String,
    pub role: Option<String>,
    pub email: String,
    pub password: String,
    pub is_active: Option<bool>,
    pub created_date: Option<NaiveDateTime>,
    pub updated_date: Option<NaiveDateTime>,
}

impl UserProfileTbl {
    pub async fn add_user(
        pool: &PgPool,
        payload: NewUserPayload,
    ) -> Result<UserProfileTbl, anyhow::Error> {
        let now = &Local::now().to_string()[0..19];
        let naive_datetime = NaiveDateTime::parse_from_str(now, "%Y-%m-%d %H:%M:%S").unwrap();
        let span = span!(Level::INFO, "add_user");
        let _enter = span.enter();

        let query = sqlx::query_as!(UserProfileTbl, r#"insert into user_profile (name, role, email, password, is_active, created_date, updated_date) values ($1, $2, $3, $4, $5, $6, $7) returning *"#, 
        payload.name,
        payload.role,
        payload.email,
        "12345",
        true,
        naive_datetime,
        naive_datetime

    ).fetch_one(pool).await?;

        Ok(query)
    }

    pub async fn get_user_by_email(
        pool: &PgPool,
        email: &str,
    ) -> Result<Option<UserProfileTbl>, anyhow::Error> {
        let span = span!(Level::INFO, "get_user_by_email");
        let _enter = span.enter();
        let user = sqlx::query_as!(
            UserProfileTbl,
            r#"SELECT * FROM user_profile WHERE email = $1"#,
            email
        )
        
        .fetch_optional(pool)
        .await?;

        Ok(user)
    }
    // get user by id
    pub async fn get_user_by_id(
        pool: &PgPool,
        id: uuid::Uuid,
    ) -> Result<Option<UserProfileTbl>, anyhow::Error> {
        let user = sqlx::query_as!(
            UserProfileTbl,
            r#"SELECT * FROM user_profile WHERE user_id = $1"#,
            id
        )
        .fetch_optional(pool)
        .await?;

        if let Some(user) = user {
            Ok(Some(user))
        } else {
            Ok(None)
        }
    }

    pub async fn update_user(
        id: uuid::Uuid,
        payload: UpdateUserPayload,
        pool: &PgPool,
    ) -> Result<Option<UserProfileTbl>, anyhow::Error> {
        let now = &Local::now().to_string()[0..19];
        let naive_datetime = NaiveDateTime::parse_from_str(now, "%Y-%m-%d %H:%M:%S").unwrap();

        let user = Self::get_user_by_id(pool, id).await?;
        let mut query = None;

        if let Some(user) = user {
            query = sqlx::query_as!(
                UserProfileTbl,
                r#"
            UPDATE user_profile
            SET name = $1, role = $2, updated_date = $3
            WHERE user_id = $4
            RETURNING *
            "#,
                payload.name.unwrap_or(user.name),
                payload.role,
                naive_datetime,
                id,
            )
            .fetch_optional(pool)
            .await?;
        }
        Ok(query)
    }
    //Delete user by id
    pub async fn delete_user_by_id(pool: &PgPool, id: uuid::Uuid) -> Result<bool, anyhow::Error> {
        let result =
            sqlx::query(r#"DELETE FROM user_profile WHERE user_id = $1 RETURNING user_id"#)
                .bind(id)
                .execute(pool)
                .await?;

        Ok(result.rows_affected() > 0)
    }
}
