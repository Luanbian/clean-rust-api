use crate::traits::repository::RepositoryTrait;

pub struct FindPriceRepo;

impl RepositoryTrait for FindPriceRepo {
    fn main(&self) -> Option<String> {
        Some(String::from("Price found successfully from repo"))
    }
}
