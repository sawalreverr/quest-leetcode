fn build_array(target: Vec<i32>, n: i32) -> Vec<String> {
    let mut log: Vec<String> = Vec::new();
    let mut idx: usize = 0;

    for i in 1..=n {
        if idx == target.len() {
            break;
        }

        log.push("Push".into());

        if target[idx] == i {
            idx += 1;
        } else {
            log.push("Pop".into());
        }
    }
    return log;
}

fn eval_rpn(tokens: Vec<String>) -> i32 {
    let mut stack: Vec<i32> = Vec::new();

    for token in tokens {
        match token.as_str() {
            "+" => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(a + b);
            }
            "-" => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(a - b);
            }
            "*" => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(a * b);
            }
            "/" => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(a / b);
            }
            _ => {
                stack.push(token.parse::<i32>().unwrap());
            }
        }
    }

    return stack.pop().unwrap();
}

fn exclusive_time(n: i32, logs: Vec<String>) -> Vec<i32> {
    let mut result = vec![0; n as usize];
    let mut stack: Vec<usize> = Vec::new();
    let mut prev_time = 0;

    for log in logs {
        let mut parts = log.split(":");
        let id: usize = parts.next().unwrap().parse().unwrap();
        let typ = parts.next().unwrap();
        let time: i32 = parts.next().unwrap().parse().unwrap();

        if let Some(&current) = stack.last() {
            result[current] += time - prev_time;
        }

        if typ == "start" {
            stack.push(id);
            prev_time = time;
        } else {
            let last = stack.pop().unwrap();
            result[last] += 1;
            prev_time = time + 1;
        }
    }

    return result;
}

fn main() {
    // Q1. Build an array with stack operations
    let target = vec![2, 3, 4];
    let result = build_array(target, 4);
    println!("{:?}", result);

    // Q2. Evaluate reverse polish notation
    let tokens = vec![
        "10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+",
    ];
    let result = eval_rpn(tokens.iter().map(|&s| s.to_string()).collect());
    println!("{:?}", result);

    // Q3. Exclusive time of functions
    let logs = vec![
        "0:start:0",
        "0:start:2",
        "0:end:5",
        "1:start:6",
        "1:end:6",
        "0:end:7",
    ];
    let result = exclusive_time(2, logs.iter().map(|&s| s.to_string()).collect());
    println!("{:?}", result);
}
