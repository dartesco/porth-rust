#[derive(Debug)]
enum Operation {
    OpPush,
    OpPlus,
    OpMinus,
    OpDump,
}

fn push(val: u64) -> (Operation, Option<u64>) {
    (Operation::OpPush, Some(val))
}

fn plus() -> (Operation, Option<u64>) {
    (Operation::OpPlus, None)
}

fn minus() -> (Operation, Option<u64>) {
    (Operation::OpMinus, None)
}

fn dump() -> (Operation, Option<u64>) {
    (Operation::OpDump, None)
}

fn simulate_program(program: Vec<(Operation, Option<u64>)>) {
    let mut stack: Vec<u64> = Vec::new();

    for op in program.iter() {
        match op.0 {
            Operation::OpPush => if let Some(x) = op.1 {
                stack.push(x);
            }
            Operation::OpPlus => {
                let a = stack.pop();
                let b = stack.pop();
                if let (Some(x), Some(y)) = (a, b) {
                    stack.push(x + y);
                }
            }
            Operation::OpMinus => {
                let a = stack.pop();
                let b = stack.pop();
                if let (Some(x), Some(y)) = (a, b) {
                    stack.push(y - x);
                }
            }
            Operation::OpDump => {
                let a = stack.pop();
                if let Some(x) = a {
                    println!("{}", x);
                }
            },
        }
    }
}

fn main() {
    let program = vec![push(2), push(4), plus(), dump(), push(2), push(1), minus(), dump()];
    simulate_program(program);
}
