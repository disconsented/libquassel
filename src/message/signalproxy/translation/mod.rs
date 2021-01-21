pub trait Network {
    type Item;

    fn to_network(&self) -> Self::Item;
    fn from_network(input: Self::Item) -> Self;
}
