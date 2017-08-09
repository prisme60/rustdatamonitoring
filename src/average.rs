
pub trait Average<T, ACC > {
    fn cumulate<'a, 'b>(&'a self, cumulated_data : &'b mut ACC) -> &'b ACC;
    fn divide(&self, cumulated_data : &ACC, nb_elements : usize) -> T;
}
