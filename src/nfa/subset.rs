use std::collections::{
    TrieSet,
};
use nfa::Nfa;
use dfa::Dfa;
use nfa::state::StateId as NfaStateId;
use dfa::state::StateId as DfaStateId;
use nfa::state::StateEdge as NfaStateEdge;

macro_rules! set(
    ($($item:expr),*) => ({
        use std::collections::TrieSet;

        let mut _set = TrieSet::new();

        $(
        _set.insert($item);
        )*

        _set
    });

    ($($item:expr),*,) => ({
        set!($($item),*)
    })
)

fn closure<T>(nfa: &Nfa<T>, states: &TrieSet) -> TrieSet {
    let mut set = TrieSet::new();

    fn inner<T>(nfa: &Nfa<T>, set: &mut TrieSet, state: NfaStateId) {
        set.insert(state);

        if let Some(state) = nfa.get_state(state) {
            for &(ref edge, ref states) in state.edges_iter() {
                if let &NfaStateEdge::Epsilon = edge {
                    for state in states.iter() {
                        if !set.contains(&state) {
                            inner(nfa, set, state);
                        }
                    }
                }
            }
        }
    }
    for state in states.iter() {
        inner(nfa, &mut set, state);
    }

    set
}

fn get_move<T: Copy + Eq>(nfa: &Nfa<T>, states: &TrieSet, edge: T) -> TrieSet {
    let mut set = TrieSet::new();

    fn inner<T: Copy + Eq>(nfa: &Nfa<T>, set: &mut TrieSet, state: NfaStateId, edge: T) {
        if let Some(state) = nfa.get_state(state) {
            if let Some(ref targets) = state.get_edges(NfaStateEdge::Value(edge)) {
                for target in targets.iter() {
                    set.insert(target);
                }
            }
        }
    }
    for state in states.iter() {
        inner(nfa, &mut set, state, edge);
    }

    set
}

fn get_edges<T: Copy + Eq>(nfa: &Nfa<T>, states: &TrieSet) -> Vec<T> {
    let mut moves = Vec::new();

    fn inner<T: Copy + Eq>(nfa: &Nfa<T>, moves: &mut Vec<T>, state: NfaStateId) {
        if let Some(state) = nfa.get_state(state) {
            for edge in state.edges_iter().map(|&(edge, _)| edge).filter_map(|edge| match edge {
                NfaStateEdge::Value(edge) => Some(edge),
                _ => None,
            }) {
                if moves.iter().find(|&e| &edge == e).is_none() {
                    moves.push(edge);
                }
            }
        }
    }
    for state in states.iter() {
        inner(nfa, &mut moves, state);
    }

    moves
}

struct DfaState {
    pub id: DfaStateId,
    states: TrieSet,
}

impl DfaState {
    pub fn new(id: DfaStateId, states: TrieSet) -> DfaState {
        DfaState {
            id: id,
            states: states,
        }
    }
}

pub fn process<T: Copy + Eq>(nfa: &Nfa<T>) -> Dfa<T> {
    let mut dfa = Dfa::new();

    let start = nfa.start.expect("No start state for Nfa!");
    let start = DfaState::new(dfa.add_state().id(), closure(nfa, &set!(start)));

    dfa.set_start(start.id);

    let mut queue = vec![start.id];
    let mut states = vec![start];

    while let Some(id) = queue.pop() {
        let edges = get_edges(nfa, &states[id].states);
        for edge in edges.into_iter() {
            let s = closure(nfa, &get_move(nfa, &states[id].states, edge));

            let sid = states.iter().find(|state| state.states == s).map(|state| state.id);
            let s = match sid {
                Some(s) => s,
                None => {
                    let s = DfaState::new(dfa.add_state().id(), s);
                    let nid = s.id;

                    queue.push(nid);
                    states.push(s);

                    nid
                }
            };

            dfa.link(id, edge, s);
        }
    }

    for state in states.into_iter() {
        for _ in state.states.intersection(&nfa.goals) {
            dfa.add_goal(state.id);
        }
    }

    dfa
}

#[cfg(test)]
mod test {
    use super::closure;
    use nfa::Nfa;
    use nfa::state::StateEdge;

    #[test]
    #[allow(non_snake_case)]
    fn test_closure() {
        let mut nfa: Nfa<char> = Nfa::new();

        let S0 = nfa.add_state().id();

        let S10 = nfa.add_state().id();
        let S17 = nfa.add_state().id();

        let S20 = nfa.add_state().id();
        let S28 = nfa.add_state().id();

        let S3 = nfa.add_state().id();

        nfa.link(S0, StateEdge::Epsilon, S10);
        nfa.link(S10, StateEdge::Value('m'), S17);
        nfa.link(S17, StateEdge::Epsilon, S3);

        nfa.link(S0, StateEdge::Epsilon, S20);
        nfa.link(S20, StateEdge::Value('c'), S28);
        nfa.link(S28, StateEdge::Epsilon, S3);

        nfa.set_start(S0);
        nfa.add_goal(S3);

        assert_eq!(closure(&nfa, &set!(S0)), set!(S0, S10, S20));
        assert_eq!(closure(&nfa, &set!(S17, S28)), set!(S17, S28, S3));
    }
}
