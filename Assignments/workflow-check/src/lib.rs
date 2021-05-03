#[cfg(test)]
mod tests {
    use crate::testWorkflow;
    #[test]
    assert_eq!(testWorkflow(1,2),5);
}

fn testWorkflow (a: i32, b: i32) -> i32 {
    a + b
}

fn testWorkflow1 (a: i32, b: i32) -> i32 {
    a + b
}
