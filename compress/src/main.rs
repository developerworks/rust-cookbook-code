mod compress;
fn main() {
    let _ = compress::gzip();
    let _ = compress::gunzip();
    println!("Hello, world!");
}
