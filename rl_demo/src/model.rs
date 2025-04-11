
pub const INITIAL_STATE: [u64; 7] = [50, 2, 0, 50, 0, 0, 0];
pub const TARGET: [u64; 7] = [0, 0, 0, 0, 0, 20, 0];

#[derive(Debug, Clone, PartialEq)]
pub struct Transition {
    pub name: String,
    pub increment: Vec<u64>,
    pub decrement: Vec<u64>,
    pub rate: f64,
    pub in_dep_graph: bool,
}

#[derive(Debug, Clone)]
pub struct DependencyGraphNode {
    pub transition: Transition,
    pub dependencies: Vec<(DependencyGraphNode, u64)>,
}

impl DependencyGraphNode {
    pub fn new(transition: Transition) -> Self {
        Self {
            transition,
            dependencies: Vec::new(),
        }
    }

    pub fn add_dependency(&mut self, node: DependencyGraphNode, weight: u64) {
        self.dependencies.push((node, weight));
    }
}

pub fn make_8react_transitions() -> Vec<Transition> {
    vec![
        // R L RL G GA GBG GD
        Transition {
            name: "R1".to_string(),
            increment: vec![1, 0, 0, 0, 0, 0, 0],
            decrement: vec![0, 0, 0, 0, 0, 0, 0],
            rate: 0.0038,
            in_dep_graph: false,
        },
        Transition {
            name: "R2".to_string(),
            increment: vec![0, 0, 0, 0, 0, 0, 0],
            decrement: vec![1, 0, 0, 0, 0, 0, 0],
            rate: 0.0004,
            in_dep_graph: false,
        },
        Transition {
            name: "R3".to_string(),
            increment: vec![0, 1, 1, 0, 0, 0, 0],
            decrement: vec![1, 1, 0, 0, 0, 0, 0],
            rate: 0.042,
            in_dep_graph: true,
        },
        Transition {
            name: "R4".to_string(),
            increment: vec![1, 0, 0, 0, 0, 0, 0],
            decrement: vec![0, 0, 1, 0, 0, 0, 0],
            rate: 0.010,
            in_dep_graph: false,
        },
        Transition {
            name: "R5".to_string(),
            increment: vec![0, 0, 0, 0, 1, 1, 0],
            decrement: vec![0, 0, 1, 1, 0, 0, 0],
            rate: 0.011,
            in_dep_graph: true,
        },
        Transition {
            name: "R6".to_string(),
            increment: vec![0, 0, 0, 0, 0, 0, 1],
            decrement: vec![0, 0, 0, 0, 1, 0, 0],
            rate: 0.100,
            in_dep_graph: false,
        },
        Transition {
            name: "R7".to_string(),
            increment: vec![0, 0, 0, 1, 0, 0, 0],
            decrement: vec![0, 0, 0, 0, 0, 1, 1],
            rate: 1050.0,
            in_dep_graph: false,
        },
        Transition {
            name: "R8".to_string(),
            increment: vec![0, 1, 0, 0, 0, 0, 0],
            decrement: vec![0, 0, 0, 0, 0, 0, 0],
            rate: 3.210,
            in_dep_graph: true,
        },
    ]
}

pub fn make_8react_graph(transitions: Vec<Transition>) -> DependencyGraphNode {
    let mut node = DependencyGraphNode::new(
        transitions[4].clone(), // R5
    );

    let dep_node = DependencyGraphNode::new(
        transitions[2].clone(), // R3
    );
    node.add_dependency(dep_node.clone(), 50);

    let dep_node = DependencyGraphNode::new(
        transitions[2].clone(), // R3
    );
    node.add_dependency(dep_node.clone(), 50);

    node
}
