#[cfg(test)]
pub mod repository;

use crate::constant;
use crate::constant::db_conn_pool;
use crate::constant::run_migrations;
use crate::constant::DB_FILE;
use crate::errors::Error;
use once_cell::sync::OnceCell as SyncCell;
use std::fs;
use std::path::Path;
use test_context::{test_context, AsyncTestContext, TestContext};
use tokio::runtime::Runtime;
use tokio::sync::OnceCell;

///tokio runtime for sync testing
pub fn runtime() -> Result<&'static Runtime, Error> {
    static RUNTIME: SyncCell<Runtime> = SyncCell::new();
    RUNTIME.get_or_try_init(|| Runtime::new().or_else(|err| Err(Error::Runtime(err.to_string()))))
}

pub struct MyAsyncContext {
    value: String,
    db_pool: &'static sqlx::Pool<sqlx::Sqlite>,
}

pub struct MyContext {
    value: String,
}

// #[async_trait]
impl AsyncTestContext for MyAsyncContext {
    async fn setup() -> MyAsyncContext {
        initialize().await;
        MyAsyncContext {
            value: "test".to_string(),
            db_pool: db_conn_pool().await.unwrap(),
        }
    }

    async fn teardown(self) {
        // Perform any teardown you wish.
    }
}

impl TestContext for MyContext {
    fn setup() -> MyContext {
        let rt = runtime().unwrap();

        rt.block_on(initialize());
        // block_on()
        MyContext {
            value: "test".to_string(),
        }
    }

    fn teardown(self) {
        // Perform any teardown you wish.
    }
}

static ONCE: OnceCell<anyhow::Result<()>> = OnceCell::const_new();

pub async fn initialize() -> &'static anyhow::Result<()> {
    let _ = env_logger::builder()
        .target(env_logger::Target::Stdout)
        .filter_level(log::LevelFilter::Trace)
        .is_test(true)
        .try_init();
    ONCE.get_or_init(|| async {
        let test_folder = ".".to_string();
        constant::app_data_path(test_folder.to_string());

        fs::remove_file(Path::new(".").join(DB_FILE))
            .unwrap_or_else(|why| error!("! {:?}", why.kind()));
        match run_migrations().await {
            Ok(_) => {
                info!("migrations done");
            }
            Err(e) => {
                error!("migrations failed: {}", e);
            }
        };

        //read sql file
        let sql = match fs::read_to_string("./db/test/data.sql") {
            Ok(sql) => sql,
            Err(_) => {
                //找不到测试数据sql文件直接退出
                panic!("test data sql file not found")
            }
        };
        debug!("sql file {}", sql);
        //insert test data
        sqlx::query(sql.as_str())
            .execute(db_conn_pool().await?)
            .await?;
        Ok(())
    })
    .await
}

#[test_context(MyContext)]
#[test]
fn test_works(ctx: &mut MyContext) {
    info!("test_works --------");
    assert_eq!(ctx.value, "test")
}
