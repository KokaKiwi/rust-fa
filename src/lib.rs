#![experimental]
#![feature(phase, macro_rules, if_let, while_let)]

#[phase(plugin, link)]
extern crate log;
extern crate graphviz;

pub mod dfa;
pub mod nfa;
