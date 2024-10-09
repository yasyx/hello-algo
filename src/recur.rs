

pub fn recur(n: i32) -> i32 {
    if n == 1 {
        return 1;
    }

    let res = recur(n - 1);

    n + res
}

pub fn tail_recur(n: i32, res: i32) -> i32 {
    if n == 0 {
        return res;
    }

    tail_recur(n - 1, res + n)
}


pub fn fib(n: i32) -> i32 {
    if n == 1 || n == 2 {
        return n - 1;
    }

    fib(n - 1) + fib(n - 2)
}


pub fn for_loop_recur(n : i32) -> i32 {
    let mut res = 0;
    let mut stack: Vec<i32> = Vec::new();

    for i in (1..=n).rev() {
        stack.push(i);
    }

    while !stack.is_empty() {
        res += stack.pop().unwrap();
    }
    res
}