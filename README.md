# Function attributes for [Optick Profiler](https://github.com/bombomby/optick)
[![Build Status](https://github.com/bombomby/optick-attr-rs/workflows/Rust/badge.svg)](https://github.com/bombomby/optick-attr-rs/actions?workflow=Rust)
[![Crates.io](https://img.shields.io/crates/v/optick-attr.svg)](https://crates.io/crates/optick-attr)
[![Docs](https://docs.rs/optick-attr/badge.svg)](https://docs.rs/optick-attr)

A set of procedural macros to simplify performance instrumentation of the code.
![](https://optick.dev/images/screenshots/optick/Optick.png)

## How to use

In `Cargo.toml` add:

```toml
[dependencies]
optick = "1.3.2"
optick_attr = "0.3.0"
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
