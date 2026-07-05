extern crate alloc;
use crate::result::Result;
use crate::x86::busy_loop_hint;
use alloc::boxed::Box;
use core::fmt::Debug;
use core::future::Future;
use core::panic::Location;
use core::pin::Pin;
use core::ptr::null;
use  core::task::Context;
use core::task::Poll;
use core::task::RawWaker;
use core::task::RawWakerVTable;
use core::task::Waker;

