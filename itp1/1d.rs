fn main() {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let s: i32 = line.trim().parse().unwrap();
    println!("{}:{}:{}", s/3600, s%3600/60, s%60);
}