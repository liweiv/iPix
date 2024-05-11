use super::{PageQuery, Repository};
use crate::domain::UserProvider;

/// UserProviderRepository Trait
/// 继承Repository, PageQuery Trait
pub trait UserProviderRepository:
    Repository<UserProvider, String> + PageQuery<UserProvider> + Sync
{
    //扩展其他方法
}
