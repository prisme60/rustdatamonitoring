
pub trait Average<D> {
    type Acc;
    fn empty_cumulator() -> Self::Acc;
    fn cumulate<'a, 'b>(&'a self, cumulated_data : &'b mut Self::Acc) -> &'b Self::Acc;
    fn divide(cumulated_data : &Self::Acc, nb_elements : usize) -> D;
}
