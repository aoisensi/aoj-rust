fn main() {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let v: Vec<&str> = line.split_whitespace().collect();

    let a = v[0].parse::<i32>().unwrap();
    let b = v[1].parse::<i32>().unwrap();
    let c = v[2].parse::<i32>().unwrap();

    if a < b && b < c {
        println!("Yes")
    } else {
        println!("No")
    }
}