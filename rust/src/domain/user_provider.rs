use sqlx::FromRow;
#[derive(Debug, Clone, FromRow)]
pub struct UserProvider {
    pub id: i32,
    pub access_key: String,
    pub secret_key: String,
    pub host: String,
    pub prefix: Option<String>,
    pub naming_rule: Option<String>,
    pub provider_id: String,
}
