//! # stack-db
//! > A (basically) infinitely stacking database that has both readonly safety and incredible write speeds at the same time.
//! 
//! ## Examples
//! ---
//! ### Example of a basic in-memory binary database
//! Here is a basic in-memory database that only deals with binary indexes and data (that uses the allocators provided by the library)
//! ```rust
//! use stack_db::prelude::*;
//! 
//! let allocator = SkdbMemAlloc; // or `SkdbDiskAlloc::new()`
//! let mut database = StackDB::new(allocator).unwrap();
//! 
//! // writing
//! database.write(256, b"hello, ").unwrap();
//! database.write(256+7, b"world").unwrap();
//! 
//! // reading
//! assert_eq!(&*database.read(256..268).unwrap(), b"hello, world");
//! 
//! // commit to save all changes
//! database.commit().unwrap();
//! 
//! // over-writting
//! database.write(256, b"H").unwrap();
//! database.write(256+7, b"W").unwrap();
//! database.write(268, b"!").unwrap();
//! 
//! // commit again
//! database.commit().unwrap();
//! 
//! // reading
//! assert_eq!(&*database.read(256..269).unwrap(), b"Hello, World!");
//! 
//! // rebase to save space
//! database.rebase(256).unwrap(); // rebase with a 256 byte buffer
//! ```

pub mod base;
pub mod errors;
pub mod prelude;
pub mod default;

#[cfg(debug_assertions)]
#[allow(dead_code)]
fn check_iter_val<T: std::fmt::Debug>(value: T) -> T {
    dbg!(&value);
    value
}
