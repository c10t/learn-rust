use std::io::Read;

fn main() {
    // input
    // N  Q
    // L1 R1 T1
    // ...
    // Lq Rq Tq
    let mut buf = String::new();

    std::io::stdin().read_to_string(&mut buf).unwrap();

    let mut iter = buf.split_whitespace();

    let n: usize = iter.next().unwrap().parse().unwrap();
    let q: usize = iter.next().unwrap().parse().unwrap();

    let lrt: Vec<(usize, usize, u64)> = (0...q).map(|_| {
        (
            iter.next().unwrap().parse().unwrap(),
            iter.next().unwrap().parse().unwrap(),
            iter.next().unwrap().parse().unwrap(),
        )
    }).collect();
}
