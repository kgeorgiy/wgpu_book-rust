#![allow(dead_code)]

use core::{cell::RefCell, str::FromStr, fmt::Debug};

pub use vertex::*;

pub mod colormap;
pub mod vertex_data;
pub mod functions;
pub mod light;
pub mod surface_data;
pub mod mvp;
mod vertex;

pub struct CmdArgs;

thread_local!(static ARGS: RefCell<Vec<String>> = RefCell::new(std::env::args().skip(1).rev().collect()));

impl CmdArgs {
    #[must_use]
    pub fn next(default: &str) -> String {
        ARGS.with(|cell| cell.borrow_mut().pop().unwrap_or(default.to_owned()))
    }

    #[must_use]
    pub fn next_known(description: &str, known: &[&str]) -> String {
        let value = Self::next(known.first().expect("at least one known variant"));
        assert!(known.iter().any(|k| value == *k), "{description}: unknown argument '{value}', expected one of {known:?}");
        println!("{description}: {value}");
        value
    }

    #[must_use]
    pub fn next_bool(description: &str, default: bool) -> bool {
        let known = if default { ["true", "false"] } else { ["false", "true"] };
        CmdArgs::next_known(description, &known).parse().expect("true of false")
    }

    #[must_use]
    pub fn has_option(name: &str) -> bool {
        ARGS.with(|cell| {
            let mut args = cell.borrow_mut();
            let index = args.iter().position(|value| value == name);
            for idx in index.iter() { args.remove(*idx); }
            println!("Has {name}: {}", index.is_some());
            index.is_some()
        })
    }

    #[must_use]
    pub fn get_option<F: FromStr>(name: &str) -> Option<F> where <F as FromStr>::Err: Debug {
        ARGS.with(|cell| {
            let mut args = cell.borrow_mut();
            let index = args.iter().position(|value| value == name);
            index.map(|idx| {
                if idx > 0 {
                    let value = args.remove(idx - 1);
                    args.remove(idx - 1);
                    println!("{name}: {value}");
                    value.parse().expect("valid value")
                } else {
                    panic!("{name} expects argument");
                }
            })
        })
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
