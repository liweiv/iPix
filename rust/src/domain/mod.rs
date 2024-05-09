mod user_provider;
pub use user_provider::UserProvider;


pub trait Model {
    fn table_name() -> String;
}