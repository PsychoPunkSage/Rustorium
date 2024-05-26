// use ::HelloMacro;
// use hello_macro_derive::HelloMacro;
use ch19_6_hello_macro::HelloMacro;
use ch19_6_hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes;

fn main() {
    /*
    Procedural Macro:
    = Take code as input, work on that code and produce code as output.
    ** Types
    - Custom derive
    - Attribute like
    - Function like
     */

    Pancakes::hello_macro();
}
