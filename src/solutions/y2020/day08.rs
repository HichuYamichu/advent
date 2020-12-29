use std::collections::HashSet;

#[derive(Debug, Clone, Copy)]
struct Op<'a> {
    code: &'a str,
    operand: isize,
}

pub fn a(input: String) -> String {
    let code = input
        .lines()
        .map(|l| l.split_whitespace())
        .map(|mut s| Op {
            code: s.next().unwrap(),
            operand: s.next().unwrap().parse().unwrap(),
        })
        .collect::<Vec<_>>();
    let mut idx = 0;
    let mut acc: isize = 0;
    let mut seen = HashSet::new();

    loop {
        let op = code.get(idx as usize).unwrap();
        if !seen.insert(idx) {
            break;
        }

        match op.code {
            "nop" => idx += 1,
            "acc" => {
                acc += op.operand;
                idx += 1;
            }
            "jmp" => idx += op.operand,
            _ => unreachable!(),
        }
    }

    acc.to_string()
}

pub fn b(input: String) -> String {
    let code = input
        .lines()
        .map(|l| l.split_whitespace())
        .map(|mut s| Op {
            code: s.next().unwrap(),
            operand: s.next().unwrap().parse().unwrap(),
        })
        .collect::<Vec<_>>();

    let mut permutation = 0;
    loop {
        let mut code = code.clone();
        let op = &code[permutation];
        match op.code {
            "jmp" => {
                code[permutation] = Op {
                    code: "nop",
                    operand: op.operand,
                }
            }
            "nop" => {
                code[permutation] = Op {
                    code: "jmp",
                    operand: op.operand,
                }
            }
            _ => {}
        }
        permutation += 1;

        let mut idx = 0;
        let mut acc: isize = 0;
        let mut seen = HashSet::new();

        struct InfiniteLoopError;

        let res = loop {
            let op = if let Some(op) = code.get(idx as usize) {
                op
            } else {
                break Ok(acc);
            };

            if !seen.insert(idx) {
                break Err(InfiniteLoopError);
            }

            match op.code {
                "nop" => idx += 1,
                "acc" => {
                    acc += op.operand;
                    idx += 1;
                }
                "jmp" => idx += op.operand,
                _ => unreachable!(),
            }
        };

        if let Ok(acc) = res {
            break acc;
        };
    }
    .to_string()
}
