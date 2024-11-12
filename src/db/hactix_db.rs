use mongodb::{Client, Database};

/// Estrutura que representa o estado da aplicação, incluindo a conexão com o banco de dados.
pub struct AppState {
    pub database: Database,
}

impl AppState {
    /// Cria uma nova instância de `AppState`.
    ///
    /// # Arguments
    ///
    /// * `client` - Um cliente MongoDB conectado.
    pub fn new(client: Client) -> Self {
        let database = client.database("hactix_db");
        Self { database }
    }
}
