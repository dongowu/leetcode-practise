fn main() {
    println!("{}", loopfn(3))
}

fn loopfn(n: i32) -> i32 {
    let mut res = 0;
    for i in 1..=n {
        res += i;
    }
    res
}
fn whilefn(n: i32) -> i32 {
    let mut res = 0;
    let mut i = 1;
    while i <= n {
        res += i;
        i += 1;
    }
    res
}
