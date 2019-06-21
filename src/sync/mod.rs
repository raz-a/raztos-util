//! # Sync
//!
//! `sync` contains functions and data structures used to synchronize multithreaded applications.
//!

/// Contains structures and functionality to use a critical section.
pub mod critical_section;

/// Contains functionality to atomically access and modify data.
pub mod atomic;

/// Contains lock structures.
pub mod lock;

/// Contains semaphore structures to represent resource allocation.
pub mod semaphore;