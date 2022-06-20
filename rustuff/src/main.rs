use rand::Rng; // 0.8.0

fn main() {
    let mut n = 1;

    while n < 100 {
    let num = rand::thread_rng().gen_range(0..1000001);
    println!("{}", num);
    n += 1; 
    }
}