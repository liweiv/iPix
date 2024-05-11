use super::DbModel;
use sqlx::FromRow;
#[derive(Debug, Clone, FromRow, Default, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct UserProvider {
    pub id: Option<String>,
    pub access_key: Option<String>,
    pub secret_key: Option<String>,
    pub host: Option<String>,
    pub prefix: Option<String>,
    pub naming_rule: Option<String>,
    pub provider_id: Option<String>,
}

impl DbModel for UserProvider {
    fn table_name() -> String {
        "user_provider".to_string()
    }
}
