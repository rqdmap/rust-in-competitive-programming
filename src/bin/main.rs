use rust_in_competitive_programming::*;

fn main() {
    let content = stdin_to_string();
    let mut inp = Parser::new(&content);

    let t = inp.read();
    let mut vec: Vec<Vec<String>> = Vec::new();
    for _t in 1..= t {
        let n = inp.read();
        let mut v: Vec<String> = Vec::new();
        for _i in 0..n {
            let x = inp.read();
            v.push(x);
        }
        vec.push(v);
    }
    println!("{:?}", vec);
}
