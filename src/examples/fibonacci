#[optick_attr::profile]
fn fibonacci(n: u32) -> u32 {
    let res = match n {
        0 => 1,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    };
    return res;
}

#[optick_attr::capture("capture_name")]
pub fn main() {
    fibonacci(10);
}