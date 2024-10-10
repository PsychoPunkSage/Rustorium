/// What should the type of _function be?
pub fn map<T, Y, Z>(input: Vec<Y>, function: T) -> Vec<Z>
where
    T: FnMut(Y) -> Z,
{
    input.into_iter().map(function).collect()
}
