use graphviz as dot;
use std::io::IoResult;
use super::Nfa;
use super::state::StateEdge;

type Node = uint;
type Edge<T> = (uint, StateEdge<T>, uint);

struct Graph<'a, T: 'a> {
    name: String,
    nfa: &'a Nfa<T>,
}

impl<'a, T> Graph<'a, T> {
    pub fn new(name: &str, nfa: &'a Nfa<T>) -> Graph<'a, T> {
        Graph {
            name: name.into_string(),
            nfa: nfa,
        }
    }
}

impl<'a, T: ::std::fmt::Show> dot::Labeller<'a, Node, Edge<T>> for Graph<'a, T> {
    fn graph_id(&'a self) -> dot::Id<'a> {
        dot::Id::new(self.name.as_slice()).ok().expect("Can't create graph name.")
    }

    fn node_id(&'a self, node: &Node) -> dot::Id<'a> {
        match dot::Id::new(format!("state_{}", node)).ok() {
            Some(id) => id,
            None => panic!("Can't create node name for node: {}", node),
        }
    }

    fn edge_label(&'a self, edge: &Edge<T>) -> dot::LabelText<'a> {
        let (_, ref edge, _) = *edge;
        dot::LabelText::LabelStr(format!("{}", edge).into_cow())
    }
}

impl<'a, T: Clone> dot::GraphWalk<'a, Node, Edge<T>> for Graph<'a, T> {
    fn nodes(&self) -> dot::Nodes<'a, Node> {
        let nodes: Vec<_> = self.nfa.states_iter().map(|(id, _)| id).collect();
        nodes.into_cow()
    }

    fn edges(&self) -> dot::Edges<'a, Edge<T>> {
        let mut edges = Vec::new();

        for state in self.nfa.states_iter().map(|(_, state)| state) {
            for &(ref edge, ref targets) in state.edges_iter() {
                for target in targets.iter() {
                    edges.push((state.id(), edge.clone(), target));
                }
            }
        }

        edges.into_cow()
    }

    fn source(&self, edge: &Edge<T>) -> Node {
        let (source, _, _) = *edge;
        source
    }

    fn target(&self, edge: &Edge<T>) -> Node {
        let (_, _, target) = *edge;
        target
    }
}

pub fn render<T: Clone + ::std::fmt::Show, W: Writer>(w: &mut W, name: &str, nfa: &Nfa<T>) -> IoResult<()> {
    let graph = Graph::new(name, nfa);
    dot::render(&graph, w)
}
