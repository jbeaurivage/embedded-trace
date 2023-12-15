# [`embedded-trace`](https://github.com/jbeaurivage/embedded-trace)

![Crates.io](https://img.shields.io/crates/v/embedded-trace)
![docs.rs](https://img.shields.io/docsrs/embedded-trace)

A `Future` tracing utility for embedded systems.

This crate aims to provide tools to measure the execution time and debug
`async` tasks and [`Future`]s for `#![no_std]` projects.

# How to use this library

Two main traits are defined: `TraceFuture` and `Instrument`.

## `TraceFuture`
`TraceFuture` extends the standard library's `Future` trait by adding
the `trace_task`, `trace_poll` and `trace_task_and_poll` methods.
These methods each take one or more types implementing `Instrument`. The
three provided methods call `on_enter` and `on_exit` when entering the
specified spans, respectively. Consult the `TraceFuture` trait
documentation for more information.

## `Instrument`
`Instrument` represents the mechanism by which `TraceFuture`'s methods
will signal when a span is entered or exited. Implement this trait on your
own types. For instance, a simple mechanism may be to set a GPIO pin HIGH
when entering the span, and setting it LOW when exiting.

## Example use

```rust
use core::future::Future;
// `TraceFuture` must be in scope in order to use its methods.
use embedded_trace::{TraceFuture, Instrument};

/// A simulated GPIO pin that prints to `stdout` instead of setting a a physical pin's electrical state
struct FakeGpio;

// Implement `Instrument` on our type
impl Instrument for FakeGpio {
    fn on_enter(&mut self) {
        println!("HIGH");
    }

    fn on_exit(&mut self) {
        println!("LOW");
    }
}

async fn trace_a_future<F: Future>(future: F){
    let mut gpio = FakeGpio;

    // Trace the task execution
    future.trace_task(&mut gpio).await;

    // Expedted output:
    // > HIGH
    // > LOW
}
```