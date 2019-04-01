// use std::io::Read;

fn main() {
    // read one line
    let mut buf = String::new();

    std::io::stdin().read_line(&mut buf).unwrap();

    let s = buf.trim_end();
    println!("input: {}", s);

    // input:
    // N Q
    let (n, m) = {
        let mut ws = s.split_whitespace();
        let n: i32 = ws.next().unwrap().parse().unwrap();
        let m: i32 = ws.next().unwrap().parse().unwrap();
        (n, m)
    };

    println!("(n, m): {} {}", n, m);
}
