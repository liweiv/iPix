use sqlx::{Pool, Sqlite};

use crate::constant::db_conn_pool;
use futures::executor::block_on;

use super::{PageQuery, Repository};
use crate::domain::{Model, UserProvider};
use crate::errors::Error;

pub struct UserProviderRepository {
    db_pool: &'static Pool<Sqlite>,
}

impl UserProviderRepository {
    pub fn new(db_pool: &'static Pool<Sqlite>) -> Self {
        UserProviderRepository { db_pool }
    }
}

impl Default for UserProviderRepository {
    // implementation goes here
    fn default() -> Self {
        let pool = block_on(db_conn_pool());
        match pool {
            Ok(pool) => UserProviderRepository { db_pool: pool },
            Err(_) => panic!("Failed to get db connection pool"),
        }
    }
}

impl Repository<UserProvider, u32> for UserProviderRepository {
    async fn find_page(
        &self,
        page_query: PageQuery,
        query: UserProvider,
    ) -> Result<super::Page<UserProvider>, crate::errors::Error> {
        todo!()
    }
    async fn find_all(&self) -> Result<Vec<UserProvider>, crate::errors::Error> {
        let sql = format!("SELECT * FROM {}", UserProvider::table_name());
        let providers = sqlx::query_as::<_, UserProvider>(&sql)
            .fetch_all(self.db_pool)
            .await
            .map_err(|e| Error::DBQueryError(e));
        providers
    }

    async fn find_by_id(&self, id: u32) -> Result<UserProvider, crate::errors::Error> {
        let sql = format!("SELECT * FROM {} WHERE id = ?", UserProvider::table_name());
        let provider = sqlx::query_as::<_, UserProvider>(&sql)
            .bind(id)
            .fetch_one(self.db_pool)
            .await
            .map_err(|e| Error::DBQueryError(e));
        provider
    }

    async fn save(&self, entity: &mut UserProvider) -> Result<bool, crate::errors::Error> {
        //save
        if entity.id == 0 {
            let sql = format!(
                r#"
            INSERT INTO {}(access_key, secret_key, host, prefix, naming_rule, provider_id)
                VALUES (?, ?, ?, ?, ?, ?)"#,
                UserProvider::table_name()
            );
            let pk = sqlx::query(&sql)
                .bind(&entity.access_key)
                .bind(&entity.secret_key)
                .bind(&entity.host)
                .bind(&entity.prefix)
                .bind(&entity.naming_rule)
                .bind(&entity.provider_id)
                .execute(self.db_pool)
                .await
                .map_err(|e| Error::DBQueryError(e))
                .unwrap()
                .last_insert_rowid();
            if pk == 0 {
                return Err(Error::DBQueryError(sqlx::Error::RowNotFound));
            }
            entity.id = pk as u32;
            return Ok(true);
        };

        //update
        let sql = format!(
            r#"
            UPDATE {}
                SET access_key = ?, secret_key = ?, host = ?, prefix = ?, naming_rule = ?, provider_id = ? 
                WHERE id = ?"#,
            UserProvider::table_name()
        );
        let ra = sqlx::query(&sql)
            .bind(&entity.access_key)
            .bind(&entity.secret_key)
            .bind(&entity.host)
            .bind(&entity.prefix)
            .bind(&entity.naming_rule)
            .bind(&entity.provider_id)
            .bind(&entity.id)
            .execute(self.db_pool)
            .await
            .map_err(|e| Error::DBQueryError(e))
            .unwrap()
            .rows_affected();
        return Ok(ra > 0);
    }

    async fn delete(&self, id: u32) -> Result<bool, crate::errors::Error> {
        let sql = format!(r#"DELETE FROM {} WHERE id = ?"#, UserProvider::table_name());
        let rows_affected = sqlx::query(&sql)
            .bind(id)
            .execute(self.db_pool)
            .await
            .map_err(|e| Error::DBQueryError(e))
            .unwrap()
            .rows_affected();
        Ok(rows_affected > 0)
    }
}