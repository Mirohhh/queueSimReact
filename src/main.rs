fn main() {
    let now = std::time::Instant::now();
    for n in 0..=40 {
        println!("{}", fibonacci_rec(n));
    }
    println!("Elapsed: {}ms", now.elapsed().as_millis());
}

// const GOLDEN_RATIO: f32 = 1.6180339887;

// fn fibonacci(n: usize) -> u128 {
//     ((GOLDEN_RATIO.powi(n as i32) / 5.0f32.sqrt()) + 0.5) as u128
// }

fn fibonacci_rec(n: usize) -> u128 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci_rec(n - 1) + fibonacci_rec(n - 2),
    }
}
