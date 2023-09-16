use rust_in_competitive_programming::*;

fn main() {
    let content = stdin_to_string();
    let mut inp = Parser::new(&content);

    let n: usize = inp.read();
    let m: usize = inp.read();
    let s: usize = inp.read();

    let mut gra = Graph::new(n, m);
    for _ in 0..m {
        let u: usize = inp.read();
        let v: usize = inp.read();
        let w: i64 = inp.read();
        gra.add_weight(u, v, w);
    }

    let res = dijkstra(&gra, s);
    for i in 1..=n {
        print!("{} ", res[i]);
    }
    println!();
}
