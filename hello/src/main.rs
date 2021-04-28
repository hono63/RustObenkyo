fn main() {
    println!("Hello, world!");
    print_str("Hello str, String.");
}

fn print_str(s0: &str) {
    let s1: String = String::from(s0);
    let s2: &str = &s1;
    let s3: String = s2.to_string();
    println!("{}", s3);
}
