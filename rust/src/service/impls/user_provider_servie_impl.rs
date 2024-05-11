use crate::domain::UserProvider;
use crate::errors::Error;
use crate::repository::{UserProviderRepository, UserProviderRepositoryImpl};
use crate::service::UserProviderService;
/// UserProviderServiceImpl
/// UserProviderService Trait 实现类
///
pub struct UserProviderServiceImpl {
    user_provider_repo: Box<dyn UserProviderRepository>,
}

/// 实现UserProviderService Trait
impl UserProviderService for UserProviderServiceImpl {
    /// 用户新增图床，关联用户和图床提供商
    async fn add(&self, provider: &mut UserProvider) -> Result<bool, Error> {
        info!("add provider for user {:?}", provider);
        self.user_provider_repo.save(provider).await
    }
}

impl UserProviderServiceImpl {
    pub fn new(user_provider_repo: Box<dyn UserProviderRepository>) -> Self {
        Self { user_provider_repo }
    }
}
impl Default for UserProviderServiceImpl {
    fn default() -> Self {
        Self::new(Box::new(UserProviderRepositoryImpl::default()))
    }
}
