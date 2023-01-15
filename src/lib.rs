//! # mmlib
//! 
//! `mmlib` is a simple backend for `mm`-based notes apps. 
//! Internally it uses `git` to track all the changes step-by-step and allow a user 
//! to get back to any state.

extern crate git2;

//
//
//
mod misc;
mod data;

//
// List of public modules
//
pub mod repo;
pub mod error;