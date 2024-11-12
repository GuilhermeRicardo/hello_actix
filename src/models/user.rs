#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub password: String,  // Senha hashada
}
