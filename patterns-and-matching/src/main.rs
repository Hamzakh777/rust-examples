// Rust doesn't require that the conditions in a series of if let, else if, else if let arms relate to each other.
// You can see that if let can also introduce shadowed variables in the same way that match arms can
// The downside of using if let expressions is that the compiler doesn’t check for exhaustiveness, whereas with match expressions it does. 
// Similar in construction to if let, the while let conditional loop allows a while loop to run for as long as a pattern continues to match.
fn main() {
    println!("Hello, world!");
}
