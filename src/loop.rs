pub fn for_loop(n: i32) -> i32 {
    let mut res = 0;
    for i in 1..=n {
        res += i;
    }
    res
}

pub fn while_loop(n: i32) -> i32 {
    let mut res = 0;
    let mut i = 0;
    while i <= n {
        res += i;
        i = i + 1;
    }
    res
}