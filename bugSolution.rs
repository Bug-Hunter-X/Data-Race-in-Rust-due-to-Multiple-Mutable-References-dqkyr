fn main() {
    let mut x = 5;
    { //Adding a scope to limit the life time of y
        let y = &mut x; 
        *y += 1;
    }
    {
        let z = &mut x; 
        *z += 1;
    }
    println!("{}", x);
}
