use test_context::test_context;

use crate::domain::UserProvider;
use crate::tests::MyAsyncContext;

use crate::repository::{PageQuery, Pageable, Repository, UserProviderRepositoryImpl};

#[test_context(MyAsyncContext)]
#[tokio::test]
async fn get_all_user_providers(ctx: &mut MyAsyncContext) {
    let user_provider_repo = UserProviderRepositoryImpl::new(&ctx.db_pool);

    let res = user_provider_repo
        .find_by_id("afe9b0cf-9571-4cd0-97a8-f126dbe808c1".to_string())
        .await;
    assert_ne!(res.is_err(), true);
    let provider = res.unwrap();
    debug!("provider: {:?}", provider);
    assert_ne!(provider.id, None);

    assert_eq!(provider.id.unwrap(), "afe9b0cf-9571-4cd0-97a8-f126dbe808c1");
}

#[test_context(MyAsyncContext)]
#[tokio::test]
async fn update_user_providers(ctx: &mut MyAsyncContext) {
    let user_provider_repo = UserProviderRepositoryImpl::new(&ctx.db_pool);

    let res = user_provider_repo
        .find_by_id("afe9b0cf-9571-4cd0-97a8-f126dbe808c1".to_string())
        .await;
    assert_ne!(res.is_err(), true);
    let mut provider = res.unwrap();
    debug!("provider: {:?}", provider);
    provider.access_key = Some("new_access_key".to_string());
    let res = user_provider_repo.save(&mut provider).await;
    debug!("updated provider: {:?}", provider);
    assert_ne!(res.is_err(), true);
    let ok = res.unwrap();
    assert!(ok)
}

#[test_context(MyAsyncContext)]
#[tokio::test]
async fn save_user_providers(ctx: &mut MyAsyncContext) {
    let user_provider_repo = UserProviderRepositoryImpl::new(&ctx.db_pool);

    let mut provider = UserProvider {
        id: None,
        access_key: Some("c".to_string()),
        secret_key: Some("c".to_string()),
        host: Some("c".to_string()),
        prefix: Option::None,
        naming_rule: Option::None,
        provider_id: Some("QINIU".to_string()),
    };
    debug!("provider: {:?}", provider);
    let res = user_provider_repo.save(&mut provider).await;
    debug!("saved provider: {:?}", provider);
    assert_ne!(res.is_err(), true);
    let ok = res.unwrap();
    assert!(ok)
}
#[test_context(MyAsyncContext)]
#[tokio::test]
async fn find_all_page(_ctx: &mut MyAsyncContext) {
    let user_provider_repo = UserProviderRepositoryImpl::default();
    let pageable = Pageable {
        page: 1,
        page_size: 10,
        sort: None,
    };
    let mut query = UserProvider::default();

    query.provider_id = Some("QINIU".to_string());

    let res = user_provider_repo.find_all_with_page(pageable, query).await;
    assert_ne!(res.is_err(), true);
    debug!("page: {:?}", res.unwrap());
}
