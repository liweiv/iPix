use sqlx::{Pool, Sqlite};

use crate::constant::db_conn_pool;
use futures::executor::block_on;

use crate::domain::{DbModel, UserProvider};
use crate::errors::Error;
use crate::repository::{
    Page, PageQuery, Pageable, Repository, UserProviderRepository, DEFAULT_PAGE_SIZE,
};

pub struct UserProviderRepositoryImpl {
    db_pool: &'static Pool<Sqlite>,
}

impl UserProviderRepositoryImpl {
    pub fn new(db_pool: &'static Pool<Sqlite>) -> Self {
        UserProviderRepositoryImpl { db_pool }
    }
}

impl Default for UserProviderRepositoryImpl {
    // implementation goes here
    fn default() -> Self {
        let pool = block_on(db_conn_pool());
        match pool {
            Ok(pool) => UserProviderRepositoryImpl { db_pool: pool },
            Err(_) => panic!("Failed to get db connection pool"),
        }
    }
}

#[async_trait]
impl Repository<UserProvider, String> for UserProviderRepositoryImpl {
    async fn find_all(&self) -> Result<Vec<UserProvider>, crate::errors::Error> {
        let sql = format!("SELECT * FROM {}", UserProvider::table_name());
        let providers = sqlx::query_as::<_, UserProvider>(&sql)
            .fetch_all(self.db_pool)
            .await
            .map_err(|e| Error::DBQueryError(e));
        providers
    }

    async fn find_by_id(&self, id: String) -> Result<UserProvider, crate::errors::Error> {
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
        if entity.id == None {
            let sql = format!(
                r#"
            INSERT INTO {}(id, access_key, secret_key, host, prefix, naming_rule, provider_id)
                VALUES (?, ?, ?, ?, ?, ?, ?)"#,
                UserProvider::table_name()
            );
            let id = uuid::Uuid::new_v4().to_string();
            let rows_affected = sqlx::query(&sql)
                .bind(&id)
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
                .rows_affected();

            entity.id = Some(id);

            return Ok(rows_affected > 0);
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

    async fn delete(&self, id: String) -> Result<bool, crate::errors::Error> {
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
#[async_trait]
impl PageQuery<UserProvider> for UserProviderRepositoryImpl {
    async fn find_all_with_page(
        &self,
        pageable: Pageable,
        query: UserProvider,
    ) -> Result<Page<UserProvider>, crate::errors::Error> {
        if (pageable.page == 0) || (pageable.page_size == 0) {
            return Err(Error::InvalidPageableError(
                "page/pageSize is empty.".to_string(),
            ));
        }
        let mut sql = format!("SELECT * FROM {} WHERE 1=1", UserProvider::table_name());

        let mut count_sql = format!(
            "SELECT COUNT(*) FROM {} WHERE 1=1 LIMIT {} OFFSET {}",
            UserProvider::table_name(),
            pageable.page_size,
            (pageable.page - 1) * DEFAULT_PAGE_SIZE
        );
        let query = &query;

        if query.provider_id.is_some() {
            sql = sql + " AND provider_id = ?";
            count_sql = count_sql + " AND provider_id = ?";
        }

        let mut bind: sqlx::query::QueryAs<Sqlite, UserProvider, sqlx::sqlite::SqliteArguments> =
            sqlx::query_as::<_, UserProvider>(&sql);

        let mut count_bind: sqlx::query::QueryAs<Sqlite, (i64,), sqlx::sqlite::SqliteArguments> =
            sqlx::query_as(&count_sql);

        if query.provider_id.is_some() {
            bind = bind.bind(query.provider_id.clone().unwrap());
            count_bind = count_bind.bind(query.provider_id.clone().unwrap());
        }

        let count: (i64,) = count_bind
            .fetch_one(self.db_pool)
            .await
            .map_err(|e| Error::DBQueryError(e))?;

        let providers = bind
            .fetch_all(self.db_pool)
            .await
            .map_err(|e| Error::DBQueryError(e))?;

        self.build_pagination(pageable, count.0, providers)
    }
}
impl UserProviderRepository for UserProviderRepositoryImpl {}
