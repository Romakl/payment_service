use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};
use uuid::Uuid;
use std::env;

#[derive(Debug, Deserialize)]
struct PaymentRequest {
    amount: f64,
    currency: String,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
struct PaymentRecord {
    transaction_id: Uuid,
    amount: f64,
    currency: String,
    status: String,
}

struct AppState {
    pool: Pool<Postgres>,
}

async fn process_payment(
    state: web::Data<AppState>,
    req: web::Json<PaymentRequest>,
) -> impl Responder {
    let transaction_id = Uuid::new_v4();
    let query = r#"
        INSERT INTO payments (transaction_id, amount, currency, status)
        VALUES ($1, $2, $3, $4)
        RETURNING transaction_id, amount, currency, status;
    "#;

    match sqlx::query_as::<_, PaymentRecord>(query)
        .bind(transaction_id)
        .bind(req.amount)
        .bind(&req.currency)
        .bind("processed")
        .fetch_one(&state.pool)
        .await
    {
        Ok(record) => HttpResponse::Ok().json(record),
        Err(e) => {
            eprintln!("DB Error: {}", e);
            HttpResponse::InternalServerError().body("Error processing payment")
        }
    }
}

async fn get_payment(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
) -> impl Responder {
    let query = r#"
        SELECT transaction_id, amount, currency, status
        FROM payments
        WHERE transaction_id = $1
    "#;

    match sqlx::query_as::<_, PaymentRecord>(query)
        .bind(*path)
        .fetch_one(&state.pool)
        .await
    {
        Ok(record) => HttpResponse::Ok().json(record),
        Err(sqlx::Error::RowNotFound) => HttpResponse::NotFound().body("Payment not found"),
        Err(e) => {
            eprintln!("DB Error: {}", e);
            HttpResponse::InternalServerError().body("Internal server error")
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    dotenvy::dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = sqlx::PgPool::connect(&database_url).await.expect("Unable to connect to DB");

    sqlx::migrate!().run(&pool).await.expect("Could not run migrations");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState { pool: pool.clone() }))
            .route("/pay", web::post().to(process_payment))
            .route("/payment/{transaction_id}", web::get().to(get_payment))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
