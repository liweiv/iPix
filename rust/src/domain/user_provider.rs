use sqlx::FromRow;

use super::Model;
#[derive(Debug, Clone, FromRow)]
pub struct UserProvider {
    pub id: u32,
    pub access_key: String,
    pub secret_key: String,
    pub host: String,
    pub prefix: Option<String>,
    pub naming_rule: Option<String>,
    pub provider_id: String,
}

impl Model for UserProvider{
    fn table_name() -> String {
        "user_provider".to_string()
    }
}