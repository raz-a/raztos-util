//! # RazTOS Utilities
//!
//! `raztos_util` is a stand-alone, no-std crate that contains useful utilities for embedded
//! systems applications.
//!
//! These utilities include:
//! * Search and Sort Algorithms with low jitter.
//!
//! * Dynamic Memory Allocators with controllable sizes and constant time
//!   (or near constant time) allocations/frees.
//!
//! * Common Item collections for efficiently storing structures with low memory footprint.
//!
//! * Compile-time CPU feature checks and instruction-set optimized procedures.
//!
//! * Synchronization primitives for multi-threaded applications.
//!
//! This crate is used by RazTOS as a foundational library.

#![cfg_attr(not(test), no_std)]

pub mod algorithms;

/// Memory Allocators with real-time guarantees.
pub mod allocators;

pub mod collections;

pub mod cpu_features;

pub mod sync;