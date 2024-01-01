fn main() {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let v: Vec<&str> = line.split_whitespace().collect();
    let w = v[0].parse::<i32>().unwrap();
    let h = v[1].parse::<i32>().unwrap();
    let x = v[2].parse::<i32>().unwrap();
    let y = v[3].parse::<i32>().unwrap();
    let r = v[4].parse::<i32>().unwrap();
    if 0 <= x-r && x+r <= w && 0 <= y-r && y+r <= h {
        println!("Yes")
    } else {
        println!("No")
    }
}