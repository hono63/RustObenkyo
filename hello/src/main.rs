fn main() {
    let a = 3;
    println!("Hello, world{}!", a);
    print_str("Hello str, String.");
    matching();
    vector_for();
    box_test();
    print_okerr(if_statement(0));
    loop_statement();
    iteration();
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

fn box_test() {
    let array = [b'h', b'e', b'l', b'o'];
    print_box(Box::new(array)); // allocated in heap memory
}
// pointer to be set
fn print_box(a: Box<[u8]>) {
    println!{"{:?}", a};
}

fn if_statement(num: i32) -> Result<i32, String> {
    if num <= 0 {
        Err("under 0.".to_string())
    } else if num < 10 {
        Ok(0)
    } else {
        Err("beyond 10.".to_string())
    }
}

fn print_okerr(ret: Result<i32, String>) {
    match ret {
        Ok(okkee) => println!("ok:{}", okkee),
        Err(eraa) => println!("error:{}", eraa),
    };
}

fn loop_statement() {
    let mut count = 0;
    'sample_loop: loop {
        println!{"count:{}", count}
        count += 1;
        if count >= 10 {
            break 'sample_loop;
        }
    }
}

fn iteration() {
    let it = Iter {
        cur: 0,
        max: 10,
    };
    for i in it {
        println!("it:{}", i);
    }
}
struct Iter {
    cur: i32,
    max: i32,
}
impl Iterator for Iter {
    type Item = i32; // himozuke
    fn next(&mut self) -> Option<i32> {
        self.cur += 1;
        if self.cur - 1 < self.max {
            Some(self.cur -1)
        } else {
            None
        }
    }
}