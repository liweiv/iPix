use test_context::test_context;

use crate::domain::UserProvider;
use crate::tests::MyAsyncContext;

use crate::repository::{Repository, UserProviderRepository};

#[test_context(MyAsyncContext)]
#[tokio::test]
async fn get_all_user_providers(ctx: &mut MyAsyncContext) {
    let user_provider_repo = UserProviderRepository::new(&ctx.db_pool);

    let res = user_provider_repo.find_by_id(1).await;
    assert_ne!(res.is_err(), true);
    let provider = res.unwrap();
    debug!("provider: {:?}", provider);
    assert_eq!(provider.id, 1);
}

#[test_context(MyAsyncContext)]
#[tokio::test]
async fn update_user_providers(ctx: &mut MyAsyncContext) {
    let user_provider_repo = UserProviderRepository::new(&ctx.db_pool);

    let res = user_provider_repo.find_by_id(1).await;
    assert_ne!(res.is_err(), true);
    let mut provider = res.unwrap();
    debug!("provider: {:?}", provider);
    provider.access_key = "new_access_key".to_string();
    let res = user_provider_repo.save(&mut provider).await;
    debug!("updated provider: {:?}", provider);
    assert_ne!(res.is_err(), true);
    let ok = res.unwrap();
    assert!(ok)
}

#[test_context(MyAsyncContext)]
#[tokio::test]
async fn save_user_providers(ctx: &mut MyAsyncContext) {
    let user_provider_repo = UserProviderRepository::new(&ctx.db_pool);

    let mut provider = UserProvider {
        id: 0,
        access_key: "c".to_string(),
        secret_key: "c".to_string(),
        host: "c".to_string(),
        prefix: Option::None,
        naming_rule: Option::None,
        provider_id: "QINIU".to_string(),
    };
    debug!("provider: {:?}", provider);
    let res = user_provider_repo.save(&mut provider).await;
    debug!("saved provider: {:?}", provider);
    assert_ne!(res.is_err(), true);
    let ok = res.unwrap();
    assert!(ok)
}
