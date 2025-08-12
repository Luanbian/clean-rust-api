pub trait RepositoryTrait: Send + Sync {
    type Input;
    type Output;

    async fn main(
        &self,
        input: Option<Self::Input>,
    ) -> Result<Self::Output, Box<dyn std::error::Error>>;
}
