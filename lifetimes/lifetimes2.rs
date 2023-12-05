// lifetimes2.rs
//
// So if the compiler is just validating the references passed to the annotated
// parameters and the return type, what do we need to change?
//
// Execute `rustlings hint lifetimes2` or use the `hint` watch subcommand for a
// hint.


fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = "long string is long";
    let result: String;
    {
        let string2 = String::from("xyz");
        result = longest(string1, string2.as_str()).to_string();
    }
    println!("The longest string is '{}'", result);
}
