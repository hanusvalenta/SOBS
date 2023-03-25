/*fn main() // Old version
{
    loop
    {
       let mut a = Vec::new();
        a.push(1);
    }
}*/

fn main() {
    let mut v = Vec::new();
    loop {
        v.push(vec![0; 1000000]); // Allocate 1MB
    }
}