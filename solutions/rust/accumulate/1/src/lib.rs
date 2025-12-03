/// Apply function to the collection in an input vector and return a vector of results.
pub fn map<T, R, F: FnMut(T) -> R>(input: Vec<T>, function: F) -> Vec<R> {
    let mut results = Vec::new();
    let mut f = function;
    for value in input {
        results.push(f(value));
    }
    results
}
