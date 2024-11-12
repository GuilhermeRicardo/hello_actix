use actix_web::{post, web, HttpResponse, Responder};
use crate::auth::jwt::generate_token;
use crate::models::user::User;

/// Endpoint para login de usuários.
///
/// # Arguments
///
/// * `user` - JSON contendo os dados do usuário (id, username, password).
///
/// # Returns
///
/// Um token JWT se o login for bem-sucedido.
#[post("/login")]
async fn login(user: web::Json<User>) -> impl Responder {
    let secret: &str = "your_secret_key"; // Normalmente você usaria uma variável de ambiente
    let token: String = match generate_token(&user.id, secret) {
        Ok(token) => token,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };
    HttpResponse::Ok().json(token)
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .service(login),
    );
}
