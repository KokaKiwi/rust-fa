use std::slice::Items;
pub use nfa::state::StateId;

pub type StateEdge<T> = T;
type StateEdges<T> = Vec<(StateEdge<T>, StateId)>;

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

    pub fn edges_iter<'a>(&'a self) -> Items<'a, (StateEdge<T>, StateId)> {
        self.edges.iter()
    }
}

impl<T: Copy + Eq> State<T> {
    pub fn get_edge(&self, edge: StateEdge<T>) -> Option<&StateId> {
        self.edges.iter().find(|&&(ref value, _)| value == &edge).map(|item| item.ref1())
    }

    pub fn get_edge_mut(&mut self, edge: StateEdge<T>) -> Option<&mut StateId> {
        self.edges.iter_mut().find(|&&(ref value, _)| value == &edge).map(|item| item.mut1())
    }

    pub fn add_edge(&mut self, edge: StateEdge<T>, id: StateId) {
        if let Some(edge) = self.get_edge_mut(edge) {
            *edge = id;
            return;
        }

        self.edges.push((edge, id));
    }
}
