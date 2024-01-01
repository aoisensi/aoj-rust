fn main() {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let mut v: Vec<i32> = line
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();
    v.sort();
    println!("{} {} {}", v[0], v[1], v[2])
}
