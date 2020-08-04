# Function attributes for [Optick Profiler](https://github.com/bombomby/optick)
A set of procedural macros to simplify performance instrumentation of the code.

## How to use

In `Cargo.toml` add:

```toml
[dependencies]
optick = "1.3.1"
optick_attr = "0.2.0"
```

## #[optick_attr::profile]
Instrument function.
Example:
```rust
#[optick_attr::profile]
fn calc() {
    // Do some stuff
}
```

## #[optick_attr::capture("capture_name")]
Generate performance capture for function.
Capture is saved to {working_dir}/capture_name(date-time).opt.
Example:
```rust
#[optick_attr::capture("capture_name")]
pub fn main() {
    calc();
}
```

## GUI

Use Optick GUI to open saved *.opt capture for further analysis:
https://github.com/bombomby/optick/releases

## Optick API
Fully compatible with [Rust Optick API](https://github.com/bombomby/optick-rs).

## Run as Administartor to collect ETW events
Optick uses ETW to collect hardware counters: switch-contexts, auto-sampling, CPU core utilization, etc.
Run your app as administrator to enable the collection of ETW events:
```
Start-Process cargo run -Verb runAs
```
