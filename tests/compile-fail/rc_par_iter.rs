extern crate rayon;

// Check that we can't use the par-iter API to access contents of an
// `Rc`.

use rayon::iter::IntoParallelIterator;
use std::rc::Rc;

fn main() {
    let x = vec![Rc::new(22), Rc::new(23)];
    let mut y = vec![];
    x.into_par_iter() //~ ERROR no method named `into_par_iter`
     .map(|rc| *rc)
     .collect_into_vec(&mut y);
}
