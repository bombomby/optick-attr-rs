# Rust procedural macro library for [Optick Profiler](https://github.com/bombomby/optick)

## How to use

In `Cargo.toml` add:

```toml
[dependencies]
optick = "1.3.1"
optick_attr = "0.0.1"
```

API:
```rust
1. #[optick_attr::profile] // Instrument marked function
2. #[optick_attr::capture("capture_name")] // Generate performance capture for the selected function and save it to {working_dir}/capture_name(date-time).opt
```

Example usage:
```rust
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
```

## GUI

Use Optick GUI to open saved *.opt capture for further analysis:
https://github.com/bombomby/optick/releases

## Optick API
Based and fully compatible with [Rust Optick API](https://github.com/bombomby/optick-rs).

## Run as Administartor to collect ETW events
Optick uses ETW to collect hardware counters: switch-contexts, auto-sampling, CPU core utilization, etc.
Run your app as administrator to enable the collection of ETW events:
```
Start-Process cargo run -Verb runAs
```
