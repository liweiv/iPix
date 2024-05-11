use futures::Future;

use crate::{domain::UserProvider, errors::Error};

pub trait UserProviderService {
    fn add(&self, provider: &mut UserProvider) -> impl Future<Output = Result<bool, Error>> + Send;

    // impl Future<Output = Result<T, Error>> + Send;
}
