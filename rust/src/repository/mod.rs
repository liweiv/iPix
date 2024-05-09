use std::future::Future;

use crate::{domain::Model, errors::Error};

mod user_provider;
pub use user_provider::UserProviderRepository;

const DEFAULT_PAGE_SIZE: u32 = 10;

/**
 * Repository trait
 */
pub trait Repository<T: Model, K> {
    /// Find entity by id
    /// # Examples
    /// ```
    /// let user = repository.find_by_id(1).await;
    ///
    /// assert_eq!(user.id, 1);
    /// ```
    fn find_by_id(&self, id: K) -> impl Future<Output = Result<T, Error>> + Send;
    fn save(&self, entity: &mut T) -> impl Future<Output = Result<bool, Error>> + Send;
    fn delete(&self, id: K) -> impl Future<Output = Result<bool, Error>> + Send;
    fn find_all(&self) -> impl Future<Output = Result<Vec<T>, Error>> + Send;
    fn find_page(
        &self,
        page_query: PageQuery,
        entity: T,
    ) -> impl Future<Output = Result<Page<T>, Error>> + Send;
}

/**
 * Page struct
 */

pub struct Page<T> {
    pub content: Vec<T>,
    pub total_elements: u32,
    pub total_pages: u32,
    pub page: u32,
    pub size: u32,
}

/**
 * PageQuery struct
 */
pub struct PageQuery {
    pub page: u32,
    pub size: u32,
    pub sort: Option<Sort>,
}

pub struct Sort {
    pub field: String,
    pub order: String,
}
