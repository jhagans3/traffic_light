# traffic_light
Simple state machine based on  
https://en.wikipedia.org/wiki/Traffic_light
and
https://blog.yoshuawuyts.com/state-machines/

visualization powered by https://github.com/iced-rs/iced

# WARP.md

This file provides guidance to WARP (warp.dev) when working with code in this repository.

## Project Overview

This is a Rust educational project demonstrating type-level state machines using traffic light rules from different countries (US, UK, Germany). It implements two approaches:
1. **Type-level state machine** (`src/main.rs`) - Uses Rust's type system to encode state transitions at compile-time, preventing invalid state changes
2. **Enum-based state machine with GUI** (`src/bin/traffic-light-iced-enum.rs`) - Runtime state machine with an iced GUI visualization

The type-level approach is the core concept: the compiler enforces valid state transitions (e.g., US traffic lights cannot transition from Green directly to Red without going through Yellow). Once a state is consumed by `transition()`, the old state becomes inaccessible (demonstrated by the commented-out code that won't compile).

## Commands

### Build and Run
- **Run the main binary (type-level state machine):**
  ```bash
  cargo run
  ```

- **Run the GUI visualization:**
  ```bash
  cargo run --bin traffic-light-iced-enum
  ```
  Or for better performance:
  ```bash
  cargo run --release --bin traffic-light-iced-enum
  ```

### Development
- **Check code without building:** `cargo check`
- **Format code:** `cargo fmt`
- **Lint with Clippy:** `cargo clippy`
- **Build optimized binary:** `cargo build --release`
- **Clean build artifacts:** `cargo clean`

## Architecture

### Type-Level State Machine (`src/main.rs`)
- **Core principle:** Uses Rust's generic types to encode states, making invalid state transitions impossible at compile-time
- **Zero-cost abstraction:** State tracking happens at compile-time with no runtime overhead
- **Substructural type system:** The `transition()` method consumes `self`, preventing use of old states
- **Country-specific rules:** Different countries (US vs UK) have different valid state transitions:
  - **US:** Green → Yellow → Red → Green
  - **UK:** Green → Amber → Red → AmberRed → Green (UK has an additional AmberRed state)
- **Invalid states:** Some combinations like `TrafficLight<US, Amber>` have no defined behaviors and cannot transition
- **Key types:**
  - `TrafficLight<Country, State>` - Generic struct parameterized by country and state
  - `HueState<Country, HueChange>` trait - Defines valid state transitions
  - Country types: `US`, `UK`
  - State types: `Green`, `Yellow`, `Amber`, `AmberRed`, `Red`

### Iced GUI Application (`src/bin/traffic-light-iced-enum.rs`)
- **Runtime state machine** using enums instead of compile-time types
- **Iced framework** for GUI with canvas-based rendering
- **Three country modes:** US, UK, and Germany with different transition sequences
- **Interactive visualization:** Buttons to switch countries and advance through states
- **Canvas rendering:** Draws a traffic light with three circles that light up based on current state

### Dependencies
- **iced 0.12.0** - GUI framework with `debug` and `canvas` features enabled for the visualization binary

## Coding Patterns

When modifying this code:
- The type-level approach is intentionally pedantic to demonstrate compile-time safety - don't "simplify" it to runtime checks
- New country types need implementations for each valid state transition
- The `#[allow(dead_code)]` on `TrafficLight.state` field is intentional - the field exists for type-level encoding but isn't accessed at runtime
- Maintain the distinction between US terminology (Yellow) and UK terminology (Amber) in type names
