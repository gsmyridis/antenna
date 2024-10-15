use actix_web::HttpResponse;
use actix_web::web::{Form, Data};
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;
use chrono::Utc;


#[derive(Debug, Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}


pub async fn subscribe(form: Form<FormData>, pool: Data<PgPool>) -> HttpResponse {
    let query_result = sqlx::query!(r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    ).execute(pool.get_ref())
    .await;

    match query_result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            println!("Failed to execute the query: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
