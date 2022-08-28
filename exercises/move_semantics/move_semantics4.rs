// move_semantics4.rs
// Refactor this code so that instead of passing `vec0` into the `fill_vec` function,
// the Vector gets created in the function itself and passed back to the main
// function.
// Execute `rustlings hint move_semantics4` or use the `hint` watch subcommand for a hint.


fn main() {
    let mut vec1 = fill_vec();

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

// `fill_vec()` no longer takes `vec: Vec<i32>` as argument
fn fill_vec() -> Vec<i32> {
    let mut vec = Vec::new();

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}

//Some errors have detailed explanations: E0061, E0423, E0425.
// For more information about an error, try `rustc --explain E0061`.
// hint                                                                                                                                        
// Stop reading whenever you feel like you have enough direction :) Or try
// doing one step and then fixing the compiler errors that result!
// So the end goal is to:
//    - get rid of the first line in main that creates the new vector
//    - so then `vec0` doesn't exist, so we can't pass it to `fill_vec`
//    - we don't want to pass anything to `fill_vec`, so its signature should
//      reflect that it does not take any arguments
//    - since we're not creating a new vec in `main` anymore, we need to create
//      a new vec in `fill_vec`, similarly to the way we did in `main`
