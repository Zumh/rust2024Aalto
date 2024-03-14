// if an expression is end in ; then it evalues to a unit
// () is the default return type from functions
fn nothing() {}
fn explicit_nothing() -> () {
        "this &str goes nowhere";
    }


fn main() {
    println!("{}", nothing() == ());
    println!("{}", explicit_nothing() == nothing());
}
