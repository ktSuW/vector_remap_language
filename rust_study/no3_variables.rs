/* # Binding and mutability
1 🌟 A variable can be used only if it has been initialized.
- To fix warning for unused variable, prepend with _ e.g. _y, _index
- mut mark a variable as mutable
*/
fn main() {
    // let x: i32 = 5;
    // let y: i32;
    let mut x = 5
    let _y;
    x += 3;
    assert_eq!(x,8);
    println!("Success!");
}