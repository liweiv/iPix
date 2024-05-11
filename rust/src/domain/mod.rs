mod user_provider;
pub use user_provider::UserProvider;


pub trait DbModel {
    fn table_name() -> String;
}