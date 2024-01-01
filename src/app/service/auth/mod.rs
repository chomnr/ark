pub mod provider;
pub mod integrity;

pub trait AuthProvider {
    fn test();
}