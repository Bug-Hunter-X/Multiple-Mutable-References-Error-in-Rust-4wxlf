fn main() {
    let mut x = 5;
    let y = &mut x; 
    let z = &mut x; //This will cause an error because you are creating multiple mutable references to the same variable
    *y = 6;
    *z = 7;
    println!("x = {}", x);
}