//! # embedded-trace
//!
//! A [`Future`] tracing utility for embedded systems.
//!
//! This crate aims to provide tools to measure the execution time and debug
//! `async` tasks and [`Future`]s for `#![no_std]` projects.
//!
//! # How to use this library
//!
//! Two main traits are defined: [`TraceFuture`] and [`Instrument`].
//!
//! ## [`TraceFuture`]
//! [`TraceFuture`] extends the standard library's [`Future`] trait by adding
//! the [`trace_task`], [`trace_poll`] and [`trace_task_and_poll`] methods.
//! These methods each take one or more types implementing [`Instrument`]. The
//! three provided methods call [`on_enter`] and [`on_exit`] when entering the
//! specified spans, respectively. Consult the [`TraceFuture`] trait
//! documentation for more information.
//!
//! ## [`Instrument`]
//! [`Instrument`] represents the mechanism by which [`TraceFuture`]'s methods
//! will signal when a span is entered or exited. Implement this trait on your
//! own types. For instance, a simple mechanism may be to set a GPIO pin HIGH
//! when entering the span, and setting it LOW when exiting.
//!
//! ## Example use
//!
//! ```
//! use core::future::Future;
//! // `TraceFuture` must be in scope in order to use its methods.
//! use embedded_trace::{TraceFuture, Instrument};
//!
//! /// A simulated GPIO pin that prints to `stdout` instead of setting a a physical pin's electrical state
//! struct FakeGpio;
//!
//! impl Instrument for FakeGpio {
//!     fn on_enter(&mut self) {
//!         println!("HIGH");
//!     }
//!
//!     fn on_exit(&mut self) {
//!         println!("LOW");
//!     }
//! }
//!
//! async fn trace_a_future<F: Future>(future: F){
//!     let mut gpio = FakeGpio;
//!
//!     // Trace the task execution
//!     future.trace_task(&mut gpio).await;
//!
//!     // Expedted output:
//!     // > HIGH
//!     // > LOW
//! }
//! ```
//!
//! [`trace_task`]: TraceFuture::trace_task
//! [`trace_poll`]: TraceFuture::trace_poll
//! [`trace_task_and_poll`]: TraceFuture::trace_task_and_poll
//! [`on_enter`]: Instrument::on_enter
//! [`on_exit`]: Instrument::on_exit

#![no_std]

use core::{future::Future, pin::Pin, task::Poll};

/// Trait extending [`Future`]. Each method takes one or more [`Instrument`]
/// parameters which dictate the mechanism used to signal when a span is entered
/// and exited. Refer to each method's documentation for more information.
pub trait TraceFuture: Future
where
    Self: Sized,
{
    /// Trace a [`Future`]'s task execution. The underlying [`Instrument`]
    /// calls [`on_enter`](Instrument::on_enter) when the future is first
    /// polled, and calls [`on_exit`](Instrument::on_exit) when it completes
    /// (returns [`Poll::Ready`]). This is useful for analyzing the total time
    /// it takes for your future to complete (note that this is different from
    /// the real CPU time the task consumes).
    fn trace_task<I: Instrument>(self, instrument: &mut I) -> TraceTaskFuture<'_, Self, I> {
        TraceTaskFuture {
            fut: self,
            instrument,
            polled_once: false,
        }
    }

    /// Trace a [`Future`] poll execution. The underlying [`Instrument`]
    /// calls [`on_enter`](Instrument::on_enter) every time prior to the
    /// underlying future being polled, and calls and calls
    /// [`on_exit`](Instrument::on_exit) when it completes (returns
    /// [`on_exit`](Instrument::on_exit) right after the [`poll`](Future::poll)
    /// call completes, regardless of whether the underlying future completed or
    /// not. This is useful for analyzing the time it takes to poll your future
    /// (ie, actual CPU time used).
    fn trace_poll<I: Instrument>(self, instrument: &mut I) -> TracePollFuture<'_, Self, I> {
        TracePollFuture {
            fut: self,
            instrument,
        }
    }

    /// The first underlying [`Instrument`] (`task_instrument`) acts exactly as
    /// [`trace_task`](TraceFuture::trace_task), and the second underlying
    /// [`Instrument`] (`poll_instrument`) acts exactly as
    /// [`trace_poll`](TraceFuture::trace_poll).
    fn trace_task_and_poll<'a, I1: Instrument, I2: Instrument>(
        self,
        task_instrument: &'a mut I1,
        poll_instrument: &'a mut I2,
    ) -> TraceTaskAndPollFuture<'a, Self, I1, I2> {
        TraceTaskAndPollFuture {
            fut: self,
            task_instrument,
            poll_instrument,
            polled_once: false,
        }
    }
}

impl<F: Future> TraceFuture for F {}

/// An [`Instrument`] is used to signal when a span is entered or exited.
pub trait Instrument {
    /// This method is called when the span is entered.
    fn on_enter(&mut self);

    /// This method is called when the span is exited.
    fn on_exit(&mut self);
}

pin_project_lite::pin_project! {
    pub struct TraceTaskFuture<'a, F, I>
    where
        F: Future,
        I: Instrument,
    {
        #[pin]
        fut: F,
        instrument: &'a mut I,
        polled_once: bool
    }
}

impl<'p, F, P> Future for TraceTaskFuture<'p, F, P>
where
    F: Future,
    P: Instrument,
{
    type Output = F::Output;

    fn poll(
        self: Pin<&mut Self>,
        cx: &mut core::task::Context<'_>,
    ) -> core::task::Poll<Self::Output> {
        let this = self.project();

        if !*this.polled_once {
            this.instrument.on_enter();
        }
        *this.polled_once = true;

        let poll_result = this.fut.poll(cx);
        match poll_result {
            Poll::Ready(c) => {
                this.instrument.on_exit();
                Poll::Ready(c)
            }
            Poll::Pending => Poll::Pending,
        }
    }
}

pin_project_lite::pin_project! {
    pub struct TracePollFuture<'a, F, I>
    where
        F: Future,
        I: Instrument,
    {
        #[pin]
        fut: F,
        instrument: &'a mut I,
    }
}

impl<'p, F, I> Future for TracePollFuture<'p, F, I>
where
    F: Future,
    I: Instrument,
{
    type Output = F::Output;

    fn poll(
        self: Pin<&mut Self>,
        cx: &mut core::task::Context<'_>,
    ) -> core::task::Poll<Self::Output> {
        let this = self.project();

        this.instrument.on_enter();
        let poll_result = this.fut.poll(cx);
        this.instrument.on_exit();

        match poll_result {
            Poll::Ready(c) => Poll::Ready(c),
            Poll::Pending => Poll::Pending,
        }
    }
}

pin_project_lite::pin_project! {
    pub struct TraceTaskAndPollFuture<'a, F, T, P>
    where
        F: Future,
        T: Instrument,
        P: Instrument
    {
        #[pin]
        fut: F,
        task_instrument: &'a mut T,
        poll_instrument: &'a mut P,
        polled_once: bool,
    }
}

impl<'p, F, I1, I2> Future for TraceTaskAndPollFuture<'p, F, I1, I2>
where
    F: Future,
    I1: Instrument,
    I2: Instrument,
{
    type Output = F::Output;

    fn poll(
        self: Pin<&mut Self>,
        cx: &mut core::task::Context<'_>,
    ) -> core::task::Poll<Self::Output> {
        let this = self.project();

        if !*this.polled_once {
            this.task_instrument.on_enter();
        }
        *this.polled_once = true;

        this.poll_instrument.on_enter();
        let poll_result = this.fut.poll(cx);
        this.poll_instrument.on_exit();

        match poll_result {
            Poll::Ready(c) => {
                this.task_instrument.on_exit();
                Poll::Ready(c)
            }
            Poll::Pending => Poll::Pending,
        }
    }
}
