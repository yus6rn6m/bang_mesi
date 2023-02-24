use std::io;
fn main() {
    let mut jawab = String::new();
    println!("Messi > você é o melhor goleiro do mundo");
    println!("User > Pertanyan : ");
    io::stdin()
        .read_line(&mut jawab)
        .expect("Input tidal valid");
    if jawab.trim() == "artinya apa bang messi" {
        bang_mesi();
    } else if jawab.trim() == "000" {
        let mut x: f64 = 3.14;
        loop {
            x *= x;
            println!("esemka infokan");
        }
    }
    println!("by. ipnu nuluk");
}
fn bang_mesi() {
    println!("Messi > ente kiper terbaik di dunia");
    println!("User > trimakasih bang messi");
}
