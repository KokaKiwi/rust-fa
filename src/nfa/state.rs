use std::collections::TrieSet;
use std::fmt;
use std::slice::Items;

pub type StateId = uint;
#[deriving(Clone, PartialEq, Eq)]
pub enum StateEdge<T> {
    Value(T),
    Epsilon,
}
type StateEdges<T> = Vec<(StateEdge<T>, TrieSet)>;

impl<T: fmt::Show> fmt::Show for StateEdge<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StateEdge::Value(ref value) => write!(f, "{}", value),
            StateEdge::Epsilon => write!(f, "<epsilon>"),
        }
    }
}

#[deriving(Clone, Show)]
pub struct State<T> {
    id: StateId,
    edges: StateEdges<T>,
}

impl<T> State<T> {
    pub fn new(id: StateId) -> State<T> {
        State {
            id: id,
            edges: Vec::new(),
        }
    }

    pub fn id(&self) -> StateId {
        self.id
    }

    pub fn edges_iter<'a>(&'a self) -> Items<'a, (StateEdge<T>, TrieSet)> {
        self.edges.iter()
    }
}

impl<T: Copy + Eq> State<T> {
    pub fn get_edges(&self, edge: StateEdge<T>) -> Option<&TrieSet> {
        self.edges.iter().find(|&&(ref value, _)| value == &edge).map(|item| item.ref1())
    }

    pub fn get_edges_mut(&mut self, edge: StateEdge<T>) -> Option<&mut TrieSet> {
        self.edges.iter_mut().find(|&&(ref value, _)| value == &edge).map(|item| item.mut1())
    }

    pub fn add_edge(&mut self, edge: StateEdge<T>, id: StateId) {
        if let Some(edges) = self.get_edges_mut(edge) {
            edges.insert(id);
            return;
        }

        let mut edges = TrieSet::new();
        edges.insert(id);

        self.edges.push((edge, edges));
    }
}
