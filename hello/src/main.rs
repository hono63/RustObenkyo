fn main() {
    let a = 3;
    println!("Hello, world{}!", a);
    print_str("Hello str, String.");
    matching();
    vector_for();
}

fn print_str(s0: &str) {
    let s1: String = String::from(s0);
    let s2: &str = &s1;
    let s3: String = s2.to_string();
    println!("{}", s3);
}

fn matching() {
    let ret:  Result<i32, String> = Ok(200);
    let _rete: Result<i32, String> = Err("sippai".to_string());
    match ret {
        Ok(okkee) => println!("ok:{}", okkee),
        Err(eraa) => println!("error:{}", eraa),
    };
}

fn vector_for() {
    let mut v = vec![1, 2, 3];
    v.push(9);
    for elem in &v {
        println!("v:{}", elem);
    }
}
