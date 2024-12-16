fn main() {
    let mut x = 5;
    { //Scope the mutable reference to prevent errors
        let y = &mut x; 
        *y = 6;
        println!("x = {}", x);
    }
    let z = x; //Clone value to avoid double mutable reference 
    let mut zz = z;
    zz = 7; 
    println!("x = {}, zz = {}", x, zz);
}