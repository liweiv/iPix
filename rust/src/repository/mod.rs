mod impls;
mod user_provider_repository;
use std::fmt::Debug;

pub use impls::UserProviderRepositoryImpl;
pub use user_provider_repository::UserProviderRepository;

use crate::{domain::DbModel, errors::Error};

const DEFAULT_PAGE_SIZE: u32 = 10;

/**
 * Repository trait
 */
#[async_trait]
pub trait Repository<T: DbModel, K> {
    /// Find entity by id
    /// # Examples
    /// ```
    /// let user = repository.find_by_id(1).await;
    ///
    /// assert_eq!(user.id, 1);
    /// ```
    async fn find_by_id(&self, id: K) -> Result<T, Error>;
    async fn save(&self, entity: &mut T) -> Result<bool, Error>;
    async fn delete(&self, id: K) -> Result<bool, Error>;
    async fn find_all(&self) -> Result<Vec<T>, Error>;
}
#[async_trait]
pub trait PageQuery<T: DbModel + Debug> {
    async fn find_all_with_page(&self, pageable: Pageable, entity: T) -> Result<Page<T>, Error>;
    fn build_pagination(
        &self,
        pageable: Pageable,
        count: i64,
        items: Vec<T>,
    ) -> Result<Page<T>, Error> {
        debug!("count: {}, items {:?}", count, items);
        if count == 0 {
            return Ok(Page {
                content: vec![],
                total_elements: 0,
                total_pages: 0,
                page: pageable.page,
                page_size: pageable.page_size,
            });
        }

        Ok(Page {
            content: items,
            total_elements: count as u32,
            total_pages: (f64::from(count as f32) / (DEFAULT_PAGE_SIZE as f64)).ceil() as u32,
            page: pageable.page,
            page_size: pageable.page_size,
        })
    }
}

/**
 * Page struct
 */

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Page<T> {
    pub content: Vec<T>,
    pub total_elements: u32,
    pub total_pages: u32,
    pub page: u32,
    pub page_size: u32,
}

/**
 * PageQuery struct
 */
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Pageable {
    pub page: u32,
    pub page_size: u32,
    pub sort: Option<Sort>,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Sort {
    pub field: String,
    pub order: String,
}
impl Default for Pageable {
    fn default() -> Self {
        Self {
            page: 1,
            page_size: DEFAULT_PAGE_SIZE,
            sort: None,
        }
    }
}
