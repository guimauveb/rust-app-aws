pub trait FromModel<M> {
    fn from_model(model: M) -> Self;
}
