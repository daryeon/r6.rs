#![crate_name = "r6"]

//! r6.rs is an attempt to implement R6RS Scheme in Rust language

#![feature(slice_patterns)]
#![feature(io)]

// This line should be at the top of the extern link list,
// because some weird compiler bug lets log imported from rustc, not crates.io log
#[macro_use] extern crate log;
#[macro_use] extern crate enum_primitive;

extern crate phf;
extern crate regex;
extern crate num;
extern crate unicode_categories;
extern crate immutable_map;

#[cfg(test)]
macro_rules! list{
    ($($x:expr),*) => (
        vec![$($x),*].into_iter().collect()
    )
}

#[cfg(test)]
macro_rules! sym{
    ($e:expr) => (
        Datum::Sym(Cow::Borrowed($e))
    )
}

#[cfg(test)]
macro_rules! num{
    ($e:expr) => (
        Datum::Num(Number::new_int($e, 0))
    )
}

/// Error values returned from parser, compiler or runtime
pub mod error;
/// Basic datum types
pub mod datum;
/// Implement eqv? primitive
pub mod eqv;
pub mod parser;
pub mod lexer;
/// Virtual machine running the bytecode
pub mod runtime;
/// Primitive functions
pub mod primitive;
/// Compiles datum into a bytecode
pub mod compiler;
/// R6RS `base` library
pub mod base;
/// Real part of the numerical tower
pub mod real;
/// Numerical tower
pub mod number;
/// Cast Datum into rust types
pub mod cast;
/// Macro implementations
pub mod syntax;
