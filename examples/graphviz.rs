#![feature(macro_rules, if_let, while_let)]
extern crate state;

use std::io::{
    Command,
    File,
};
use state::{
    nfa,
    dfa,
};
use state::nfa::{
    Nfa,
    subset,
};
use state::nfa::state::{
    StateEdge,
};

#[allow(non_snake_case)]
fn main() {
    let mut nfa: Nfa<char> = Nfa::new();

    let S0 = nfa.add_state().id();

    let S10 = nfa.add_state().id();
    let S11 = nfa.add_state().id();
    let S12 = nfa.add_state().id();
    let S13 = nfa.add_state().id();
    let S14 = nfa.add_state().id();
    let S15 = nfa.add_state().id();
    let S16 = nfa.add_state().id();
    let S17 = nfa.add_state().id();

    let S20 = nfa.add_state().id();
    let S21 = nfa.add_state().id();
    let S22 = nfa.add_state().id();
    let S23 = nfa.add_state().id();
    let S24 = nfa.add_state().id();
    let S25 = nfa.add_state().id();
    let S26 = nfa.add_state().id();
    let S27 = nfa.add_state().id();
    let S28 = nfa.add_state().id();

    let S3 = nfa.add_state().id();

    nfa.link(S0, StateEdge::Epsilon, S10);

    nfa.link(S10, StateEdge::Value('m'), S11);
    nfa.link(S11, StateEdge::Value('e'), S12);
    nfa.link(S12, StateEdge::Value('e'), S12);
    nfa.link(S12, StateEdge::Value('t'), S20);
    nfa.link(S12, StateEdge::Value('c'), S13);
    nfa.link(S13, StateEdge::Value('h'), S14);
    nfa.link(S13, StateEdge::Epsilon, S15);
    nfa.link(S14, StateEdge::Value('a'), S15);
    nfa.link(S15, StateEdge::Value('n'), S16);
    nfa.link(S16, StateEdge::Value('t'), S17);
    nfa.link(S16, StateEdge::Value('o'), S13);

    nfa.link(S17, StateEdge::Epsilon, S3);

    nfa.link(S0, StateEdge::Epsilon, S20);

    nfa.link(S20, StateEdge::Value('c'), S21);
    nfa.link(S21, StateEdge::Value('r'), S22);
    nfa.link(S22, StateEdge::Value('i'), S23);
    nfa.link(S23, StateEdge::Value('m'), S24);
    nfa.link(S24, StateEdge::Value('i'), S25);
    nfa.link(S25, StateEdge::Value('n'), S26);
    nfa.link(S26, StateEdge::Value('e'), S27);
    nfa.link(S27, StateEdge::Value('l'), S28);

    nfa.link(S28, StateEdge::Epsilon, S3);

    nfa.set_start(S0);
    nfa.add_goal(S3);

    let mut f = File::create(&Path::new("nfa.dot"));
    nfa::graph::render(&mut f, "NFA", &nfa).unwrap();

    Command::new("dot").arg("-Tsvg").arg("-O").arg("nfa.dot").spawn().unwrap();

    let dfa = subset::process(&nfa);

    let mut f = File::create(&Path::new("dfa.dot"));
    dfa::graph::render(&mut f, "DFA", &dfa).unwrap();

    Command::new("dot").arg("-Tsvg").arg("-O").arg("dfa.dot").spawn().unwrap();
}
