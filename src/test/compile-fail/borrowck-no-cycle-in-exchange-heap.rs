enum cycle {
    node({mut a: ~cycle}),
    empty
}
fn main() {
    let x = ~node({mut a: ~empty});
    // Create a cycle!
    match check *x { //~ NOTE loan of immutable local variable granted here
      node(ref y) => {
        y.a <- x; //~ ERROR moving out of immutable local variable prohibited due to outstanding loan
      }
    };
}
