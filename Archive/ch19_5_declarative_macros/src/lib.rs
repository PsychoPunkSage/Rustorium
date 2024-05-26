#[macro_export]
macro_rules! vec {
    /*
    Expression Matching:
    - $x:expr ::> Store any expression that comes inside $x
    - ,       ::> A literal ,(comma) may come while parsing the expression.
    - *       ::> Means pattern may match more than 0(zero) times.
    */
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            /* Generate `temp_vec.push($x)` for every match we get.
               Something like this:
                - temp_vec.push(1);
                - temp_vec.push(2);
                - temp_vec.push(3);
            */
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}
