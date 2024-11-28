pub mod circuit;

fn main() {
    // Example usage
    use circuit::ArithmeticCircuit;
    let expr = "(x + y) * y";
    let mut circuit = ArithmeticCircuit::from_expression(expr);
    let result = circuit.evaluate(&[("x", 2), ("y", 3)]);
    println!("Result: {}", result); // Should print 15
}
