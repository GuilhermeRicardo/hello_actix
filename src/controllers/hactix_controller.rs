use actix_web::{get, patch, web, HttpRequest, HttpResponse, Responder};
use futures::stream::StreamExt;
use log::info;
use mongodb::{Client, bson::doc};
use crate::db::hactix_db::AppState;
use crate::auth::jwt::verify_token;

// Função auxiliar para verificar o token
fn validate_token(req: &HttpRequest, secret: &str) -> Result<(), HttpResponse> {
    let token = req.headers().get("Authorization").ok_or_else(|| {
        HttpResponse::Unauthorized().body("Missing Authorization header")
    })?.to_str().map_err(|_| {
        HttpResponse::Unauthorized().body("Invalid Authorization header format")
    })?;

    verify_token(token, secret).map_err(|_| HttpResponse::Unauthorized().body("Invalid token"))?;
    Ok(())
}

// Função auxiliar para inserir dados no MongoDB
async fn insert_data(req_body: String, app_state: &web::Data<AppState>) -> Result<HttpResponse, HttpResponse> {
    let collection = app_state.database.collection::<bson::Document>("hactix_collection");

    let doc = doc! { "data": req_body };
    info!("Document to insert: {:?}", doc);

    collection.insert_one(doc, None).await.map_err(|e| {
        info!("Error inserting data: {:?}", e);
        HttpResponse::InternalServerError().body("Failed to update data")
    })?;

    Ok(HttpResponse::Ok().body("Data updated successfully"))
}

// Função auxiliar para buscar dados do MongoDB
async fn fetch_data(app_state: &web::Data<AppState>) -> Result<HttpResponse, HttpResponse> {
    let collection = app_state.database.collection::<bson::Document>("hactix_collection");
    let mut cursor = collection.find(None, None).await.map_err(|e| {
        info!("Error finding data: {:?}", e);
        HttpResponse::InternalServerError().body("Failed to fetch data")
    })?;

    let mut data = Vec::new();
    while let Some(result) = cursor.next().await {
        match result {
            Ok(document) => data.push(document),
            Err(e) => {
                info!("Error iterating through data: {:?}", e);
                return Err(HttpResponse::InternalServerError().body("Failed to fetch data"));
            }
        }
    }

    Ok(HttpResponse::Ok().json(data))
}

#[patch("/v1/update")]
async fn update(req: HttpRequest, req_body: String, app_state: web::Data<AppState>) -> impl Responder {
    let secret = "your_secret_key"; // Normalmente uma variável de ambiente

    if let Err(response) = validate_token(&req, secret) {
        return response;
    }

    insert_data(req_body, &app_state).await.unwrap_or_else(|e| e)
}

#[get("/v1/data")]
async fn get_data(req: HttpRequest, app_state: web::Data<AppState>) -> impl Responder {
    let secret = "your_secret_key"; // Normalmente uma variável de ambiente

    if let Err(response) = validate_token(&req, secret) {
        return response;
    }

    fetch_data(&app_state).await.unwrap_or_else(|e| e)
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(update)
       .service(get_data);
}
