pub trait UseCaseTrait {
    type Input;
    type Output;

    fn perform(&self, input: Option<Self::Input>) -> Self::Output;
}
