use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let vs = vec!["a", "b", "c", "b", "c", "d"];
    let num = rng.gen_range(42, 179);
    let q = vs[rng.gen_range(0, 5)];
    println!("{}", format!("Lets challenge https://atcoder.jp/contests/abc{0: >03}/tasks/abc{1: >03}_{2:}", num, num, q));
}
