use arithmetic_circuit::circuit::ArithmeticCircuit;

fn main() {
    let expr = "(x+y)";
    let mut circuit = ArithmeticCircuit::from_expression(expr);
    let result = circuit.evaluate(&[("x", 2), ("y", 5)]);
    println!("Result: {}", result); // Should print 15
    let solidity_code = circuit.to_solidity();
    println!("{}", solidity_code);
}
