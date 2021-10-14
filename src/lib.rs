extern crate tensorflow;
extern crate tch;

use std::ops::Add;
use std::ops::Mul;
use std::rc::Rc;

// TODO: Right now these are just planar graphs. Implement constuctors
// for non-planar graphs.
#[derive(Debug)]
pub enum Graph<T, A> {
    Vert(Rc<Vec<T>>, Rc<Vec<T>>, Rc<A>),
    Edge(),
    Beside(usize, usize, Rc<Self>, Rc<Self>),
    Before(usize, usize, Rc<Self>, Rc<Self>),
    Empty(),
}

impl<T, A> Clone for Graph<T, A> {
    fn clone(&self) -> Self {
        match self {
            Graph::Vert(inputs, outputs, label) => {
                Graph::Vert(Rc::clone(inputs), Rc::clone(outputs), Rc::clone(label))
            }
            Graph::Edge() => Graph::Edge(),
            Graph::Beside(m, n, x, y) => Graph::Beside(*m, *n, Rc::clone(x), Rc::clone(y)),
            Graph::Before(m, n, x, y) => Graph::Before(*m, *n, Rc::clone(x), Rc::clone(y)),
            Graph::Empty() => Graph::Empty(),
        }
    }
}

impl<T, A> Graph<T, A> {

    pub fn entries_and_exits(&self) -> (usize, usize) {
        match self {
            Graph::Vert(inputs, outputs, _) => (inputs.len(), outputs.len()),
            Graph::Edge() => (1, 1),
            Graph::Beside(m, n, _, _) => (*m, *n),
            Graph::Before(m, n, _, _) => (*m, *n),
            Graph::Empty() => (0, 0),
        }
    }
    
    pub fn vert(inputs : Vec<T>, outputs: Vec<T>, label: A) -> Graph<T, A> {
        Graph::Vert(Rc::new(inputs), Rc::new(outputs), Rc::new(label))
    }
}

impl<T, A> Add for Graph<T, A> {
    type Output = Graph<T, A>;

    fn add(self, other: Graph<T, A>) -> Graph<T, A> {
        let (m, _) = self.entries_and_exits();
        let (_, n) = other.entries_and_exits();
        Graph::Before(m, n, Rc::new(self), Rc::new(other))
    }
}

impl<'a, T, A> Add<&'a Graph<T, A>> for &'a Graph<T, A> {
    type Output = Graph<T, A>;

    fn add(self, other: &'a Graph<T, A>) -> Graph<T, A> {
        self.clone() + other.clone()
    }
}

impl<'a, T, A> Add<&'a Graph<T, A>> for Graph<T, A> {
    type Output = Graph<T, A>;

    fn add(self, other: &'a Graph<T, A>) -> Self {
        self + other.clone()
    }
}

impl<'a, T, A> Add<Graph<T, A>> for &'a Graph<T, A> {
    type Output = Graph<T, A>;

    fn add(self, other: Graph<T, A>) -> Graph<T, A> {
        self.clone() + other
    }
}

impl<T, A> Mul for Graph<T, A> {
    type Output = Graph<T, A>;

    fn mul(self, other: Graph<T, A>) -> Graph<T, A> {
        let (m, n) = self.entries_and_exits();
        let (p, q) = other.entries_and_exits();
        Graph::Beside(m + p, n + q, Rc::new(self), Rc::new(other))
    }
}

impl<'a, T, A> Mul<&'a Graph<T, A>> for &'a Graph<T, A> {
    type Output = Graph<T, A>;

    fn mul(self, other: Self) -> Graph<T, A> {
        self.clone() * other.clone()
    }
}

impl<'a, T, A> Mul<&'a Graph<T, A>> for Graph<T, A> {
    type Output = Graph<T, A>;

    fn mul(self, other: &'a Graph<T, A>) -> Graph<T, A> {
        self * other.clone()
    }
}

impl<'a, T, A> Mul<Graph<T, A>> for &'a Graph<T, A> {
    type Output = Graph<T, A>;

    fn mul(self, other: Graph<T, A>) -> Graph<T, A> {
        self.clone() * other
    }
}

impl<'a, T, A> Mul<&'a Graph<T, A>> for usize {
    type Output = Graph<T, A>;

    fn mul(self, graph: &Graph<T, A>) -> Graph<T, A> {
        match self {
            0 => Graph::Empty(),
            n => graph * ((n - 1) * graph),
        }
    }
}

impl<T, A> Mul<Graph<T, A>> for usize {
    type Output = Graph<T, A>;

    fn mul(self, graph: Graph<T, A>) -> Graph<T, A> {
        match self {
            0 => Graph::Empty(),
            n => &graph * ((n - 1) * &graph),
        }
    }
}
