#![allow(dead_code)]

use core::cell::RefCell;

pub use vertex::*;

pub mod colormap;
pub mod vertex_data;
pub mod functions;
pub mod light;
pub mod surface_data;
pub mod mvp;
mod vertex;

pub(crate) struct CmdArgs;

thread_local!(static ARGS: RefCell<Vec<String>> = RefCell::new(std::env::args().skip(1).rev().collect()));

impl CmdArgs {
    pub(crate) fn next(default: &str) -> String {
        ARGS.with(|cell| cell.borrow_mut().pop().unwrap_or(default.to_owned()))
    }

    pub(crate) fn next_known(description: &str, known: &[&str]) -> String {
        let value = Self::next(known.first().expect("at least one known variant"));
        assert!(known.iter().any(|k| value == *k), "{description}: unknown argument '{value}', expected one of {known:?}");
        value
    }

    pub fn next_bool(description: &str, default: bool) -> bool {
        let known = if default { ["true", "false"] } else { ["false", "true"] };
        CmdArgs::next_known(description, &known).parse().expect("true of false")
    }

    pub(crate) fn is(expected: &str) -> bool {
        ARGS.with(|cell| {
            let mut args = cell.borrow_mut();
            let result = args.starts_with(&[expected.to_owned()]);
            if result {
                args.pop();
            }
            result
        })
    }
}
