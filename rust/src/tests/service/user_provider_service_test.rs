use test_context::test_context;

use crate::{
    domain::UserProvider,
    service::{UserProviderService, UserProviderServiceImpl},
    tests::MyAsyncContext,
};

#[test_context(MyAsyncContext)]
#[tokio::test]
async fn add_user_provider(_ctx: &mut MyAsyncContext) {
    let user_provider_service = UserProviderServiceImpl::default();

    let mut provider = UserProvider {
        id: None,
        access_key: Some("test".to_string()),
        secret_key: Some("sk".to_string()),
        host: Some("ab.com".to_string()),
        prefix: Some("ab".to_string()),
        naming_rule: Some("cd".to_string()),
        provider_id: Some("QINIU".to_string()),
    };
    let res = user_provider_service.add(&mut provider).await;
    assert_ne!(res.is_err(), true);
    let ok = res.unwrap();
    assert!(ok);
    debug!("provider: {:?}", provider);
    assert!(provider.id.is_some());
}
