use std::collections::HashMap;

#[derive(Debug)]
pub enum Gate {
    Input(String),
    Add(Box<Node>, Box<Node>),
    Mul(Box<Node>, Box<Node>),
    Const(i32),
}

#[derive(Debug)]
pub struct Node {
    gate: Gate,
    value: Option<i32>,
}

impl Node {
    pub fn new(gate: Gate) -> Self {
        Node { gate, value: None }
    }

    pub fn evaluate(&mut self, inputs: &HashMap<String, i32>) -> i32 {
        if let Some(val) = self.value {
            return val;
        }
        let result = match &self.gate {
            Gate::Input(name) => *inputs.get(name).unwrap(),
            Gate::Add(left, right) => left.evaluate(inputs) + right.evaluate(inputs),
            Gate::Mul(left, right) => left.evaluate(inputs) * right.evaluate(inputs),
            Gate::Const(val) => *val,
        };
        self.value = Some(result);
        result
    }
}

pub struct ArithmeticCircuit {
    root: Node,
}

impl ArithmeticCircuit {
    pub fn new(root: Node) -> Self {
        ArithmeticCircuit { root }
    }

    pub fn from_expression(expr: &str) -> Self {
        // For simplicity, this example will manually create the circuit.
        // You can replace this with a proper expression parser.
        let x = Node::new(Gate::Input("x".to_string()));
        let y = Node::new(Gate::Input("y".to_string()));
        let add = Node::new(Gate::Add(Box::new(x), Box::new(y)));
        let mul = Node::new(Gate::Mul(
            Box::new(add),
            Box::new(Node::new(Gate::Input("y".to_string()))),
        ));
        ArithmeticCircuit::new(mul)
    }

    pub fn evaluate(&mut self, inputs: &[(&str, i32)]) -> i32 {
        let input_map: HashMap<String, i32> = inputs
            .iter()
            .cloned()
            .map(|(k, v)| (k.to_string(), v))
            .collect();
        self.root.evaluate(&input_map)
    }
}
