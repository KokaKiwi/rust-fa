use std::collections::{
    TrieMap,
    TrieSet,
};
use std::collections::trie_map::Entries;
use self::state::{
    StateEdge,
    StateId,
    State,
};

pub mod graph;
pub mod state;

#[deriving(Clone, Show)]
pub struct Dfa<T> {
    states: TrieMap<State<T>>,
    start: Option<StateId>,
    goals: TrieSet,
}

impl<T> Dfa<T> {
    pub fn new() -> Dfa<T> {
        Dfa {
            states: TrieMap::new(),
            start: None,
            goals: TrieSet::new(),
        }
    }

    pub fn add_state(&mut self) -> &mut State<T> {
        let mut id = self.states.len();

        while let Some(_) = self.get_state(id) {
            id += 1;
        }

        let state = State::new(id);
        self.states.insert(id, state);

        &mut self.states[id]
    }

    pub fn set_start(&mut self, id: StateId) {
        if self.states.contains_key(&id) {
            self.start = Some(id);
        }
    }

    pub fn get_start(&self) -> Option<&State<T>> {
        self.start.and_then(|id| self.get_state(id))
    }

    pub fn add_goal(&mut self, id: StateId) {
        if self.states.contains_key(&id) {
            self.goals.insert(id);
        }
    }

    pub fn get_goals(&self) -> Vec<&State<T>> {
        self.goals.iter().map(|id| self.get_state(id)).filter_map(|state| state).collect()
    }

    pub fn get_state(&self, id: StateId) -> Option<&State<T>> {
        self.states.get(&id)
    }

    pub fn get_state_mut(&mut self, id: StateId) -> Option<&mut State<T>> {
        self.states.get_mut(&id)
    }

    pub fn states_iter<'a>(&'a self) -> Entries<'a, State<T>> {
        self.states.iter()
    }
}

impl<T: Copy + Eq> Dfa<T> {
    pub fn link(&mut self, from: StateId, edge: StateEdge<T>, to: StateId) {
        if let Some(from) = self.get_state_mut(from) {
            from.add_edge(edge, to);
        }
    }
}
