pub trait RepositoryTrait: Send + Sync {
    fn main(&self) -> Option<String>;
}
