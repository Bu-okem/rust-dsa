use std::{cmp::min, collections::HashMap};

// fn fib_helper(n: u32, memo: &mut HashMap<u32, u64>) -> u64 {
//     memo.entry(n).or_insert((n - 1) as u64 + (n - 2) as u64);

//     (n - 1) as u64 + (n - 2) as u64
// }

fn fib_helper(n: u32, memo: &mut HashMap<u32, u64>) -> u64 {
    if n <= 1 {
        return n as u64;
    }
    match memo.get(&n) {
        Some(v) => *v,
        None => {
            let v = fib_helper(n - 1, memo) + fib_helper(n - 2, memo);
            memo.insert(n, v);

            v
        }
    }
}

fn fib_memo(n: u32) -> u64 {
    let mut memo = HashMap::new();
    fib_helper(n, &mut memo)
}

fn fib_tab(n: u32) -> u64 {
    let mut memo: Vec<u64> = vec![0, 1];
    let n = n as usize;

    for i in 2..=n {
        memo.push(memo[i - 1] + memo[i - 2])
    }

    memo[n]
}

fn coin_change(coins: &[usize], amount: usize) -> i32 {
    let mut dp = vec![amount + 1; amount + 1];

    dp[0] = 0;

    for i in 1..=amount {
        for coin in coins {
            if *coin <= i {
                dp[i] = min(dp[i], dp[i - coin] + 1);
            }
        }
    }

    if dp[amount] == amount + 1 {
        -1
    } else {
        dp[amount] as i32
    }
}
