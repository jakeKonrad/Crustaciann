// Directed Acyclic Multigraphs from
// Jeremy Gibbons, "An Initial Algebra Approach to
// Directed Graphs" but without the empty graph
enum DAMG<A> {
    Edge(u64),
    Vert(u64, u64, A),
    Seq(u64, u64, Box<Self>, Box<Self>),
    Par(u64, u64, Box<Self>, Box<Self>),
    Swap(u64, u64),
}

// Smart constructors for DAMGs.
impl<A> DAMG<A> {
    fn entries_n_exits(&self) -> (u64, u64) {
        match *self {
            Self::Edge(n) => (1, n),
            Self::Vert(m, n, _) => (m, n),
            Self::Seq(m, n, _, _) => (m, n),
            Self::Par(m, n, _, _) => (m, n),
            Self::Swap(m, n) => (m + n, n + m),
        }
    }

    fn edge(n : u64) -> Self {
        Self::Edge(n)
    }

    fn vert(m: u64, n: u64, a: A) -> Self {
        Self::Vert(m, n, a)
    }

    fn seq(x: Self, y: Self) -> Option<Self> {
        let (m, n) = x.entries_n_exits();
        let (p, q) = y.entries_n_exits();

        (n == p).then(|| Self::Seq(m, q, Box::new(x), Box::new(y)))
    }

    fn par(x: Self, y: Self) -> Self {
        let (m, n) = x.entries_n_exits();
        let (p, q) = y.entries_n_exits();

        Self::Par(m + p, n + q, Box::new(x), Box::new(y))
    }

    fn swap(m: u64, n: u64) -> Self {
        Self::Swap(m, n)
    }
}

pub struct NN<A>(DAMG<fn(A) -> A>);

// simple feedforward neural network constructors
impl<A> NN<A> {
    pub fn edge(n : u64) -> Self {
        Self(DAMG::edge(n))
    }

    pub fn neuron(m: u64, n: u64, activation: fn(A) -> A) -> Self {
       Self(DAMG::vert(m, n, activation))
    }

    pub fn seq(Self(x) : Self, Self(y) : Self) -> Option<Self> {
        DAMG::seq(x, y).map(|z| Self(z))
    }

    pub fn par(Self(x) : Self, Self(y) : Self) -> Self {
        Self(DAMG::par(x, y))
    }

    pub fn swap(m : u64, n : u64) -> Self {
        Self(DAMG::swap(m, n))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // A simple xor network
    //
    //            +-------+
    // Input A ---+       +--- Neuron A ---+    
    //            +--- ---+                +
    //                X                    +--- Output A
    //            +--- ---+                +
    // Input B ---+       +--- Neuron B ---+
    //            +-------+
    //             
    #[test]
    fn xor() {
        let inputs : NN<f32> = NN::par(NN::edge(2), NN::edge(2));
        let swap : NN<f32> = NN::par(NN::par(NN::edge(1), NN::swap(1, 1)), NN::edge(1));
        let hidden = NN::par(NN::<f32>::neuron(2, 1, |x| x.tanh()), NN::<f32>::neuron(2, 1, |x| x.tanh()));
        let output = NN::<f32>::neuron(2, 1, |x| x.tanh());
        let nn : Option<NN<f32>> = NN::seq(inputs, swap)
            .and_then(|nn| NN::seq(nn, hidden))
            .and_then(|nn| NN::seq(nn, output));
        
        assert!(nn.is_some());
    }
}
