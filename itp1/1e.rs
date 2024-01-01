fn main() {
    let scan = std::io::stdin();
    let mut line = String::new();
    scan.read_line(&mut line).unwrap();
    let v: Vec<&str> = line.split_whitespace().collect();

    let a = v[0].parse::<i32>().unwrap();
    let b = v[1].parse::<i32>().unwrap();

    if a > b {
        println!("a > b");
    } else if a < b {
        println!("a < b");
    } else {
        println!("a == b");
    }
}