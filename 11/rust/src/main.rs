use std::collections::HashMap;
use std::fs;
use std::str::SplitWhitespace;

const NUM_ROUNDS: usize = 10000;
const INPUT_FILEPATH: &str = "../test";

const MONKEY_REGEX: &str = r"Monkey (\d+):
  Starting items: (.+)
  Operation: new = (.+)
  Test: divisible by (\d+)
    If true: throw to monkey (\d)
    If false: throw to monkey (\d)
";

#[derive(Debug)]
enum Expression {
    Identifier(String),
    Constant(u64),
    Addition(Box<Expression>, Box<Expression>),
    Multiplication(Box<Expression>, Box<Expression>),
}

impl Expression {
    fn from_string(string: &str) -> Self {
        // no priority of operations, will build ast right-to-left
        let mut tokens = string.split_whitespace();
        Self::from_tokens(&mut tokens)
    }

    fn from_tokens(tokens: &mut SplitWhitespace) -> Self {
        let token = tokens.next().expect("expected token");
        let expr1 = match token.parse().ok() {
            Some(i) => Self::Constant(i),
            None => Self::Identifier(token.to_string()),
        };
        if let Some(token) = tokens.next() {
            let expr2 = Self::from_tokens(tokens);
            match token {
                "*" => Self::Multiplication(Box::new(expr1), Box::new(expr2)),
                "+" => Self::Addition(Box::new(expr1), Box::new(expr2)),
                _ => panic!("unsupported token"),
            }
        } else {
            expr1
        }
    }

    fn evaluate(&self, context: &HashMap<String, u64>, modulo: u64) -> u64 {
        match self {
            Expression::Identifier(ident) => context[ident],
            Expression::Constant(i) => *i,
            Expression::Addition(e1, e2) => {
                e1.evaluate(context, modulo) + e2.evaluate(context, modulo)
            }
            Expression::Multiplication(e1, e2) => {
                let a = e1.evaluate(context, modulo);
                let b = e2.evaluate(context, modulo);
                a * b % modulo
            }
        }
    }
}

trait Test {
    fn get_monkey(&self, worry_level: u64) -> usize;
}

#[derive(Debug)]
struct DivisibilityTest {
    divider: u64,
    monkey_id_true: usize,
    monkey_id_false: usize,
}

impl Test for DivisibilityTest {
    fn get_monkey(&self, worry_level: u64) -> usize {
        if worry_level % self.divider == 0 {
            self.monkey_id_true
        } else {
            self.monkey_id_false
        }
    }
}

#[derive(Debug)]
struct Monkey {
    operation: Expression,
    test: DivisibilityTest,
}

fn run(monkeys_by_id: &Vec<Monkey>, items_by_monkey: &mut HashMap<usize, Vec<u64>>) {
    let mut inspection_counts: Vec<usize> = vec![0; items_by_monkey.len()];
    // all values in the data are primes, so no need to do find lcm
    let modulo = monkeys_by_id
        .iter()
        .map(|m| m.test.divider)
        .fold(1, |acc, x| acc * x);

    for _round in 0..NUM_ROUNDS {
        for (monkey_id, monkey) in monkeys_by_id.iter().enumerate() {
            let items = match items_by_monkey.remove(&monkey_id) {
                Some(v) => v,
                None => vec![],
            };

            inspection_counts[monkey_id] += items.len();

            for mut item_worry_level in items {
                let mut context = HashMap::new();
                context.insert("old".to_string(), item_worry_level);

                item_worry_level = monkey.operation.evaluate(&context, modulo);

                let to_monkey_id = monkey.test.get_monkey(item_worry_level);
                let entry = items_by_monkey.entry(to_monkey_id);
                entry.or_insert(vec![]).push(item_worry_level);
            }
        }
    }

    inspection_counts.sort();
    inspection_counts.reverse();

    println!(
        "ans {:?}",
        inspection_counts.iter().take(2).fold(1, |a, &b| { a * b })
    );
}

fn main() {
    let monkey_regex = regex::Regex::new(MONKEY_REGEX).unwrap();

    let input = fs::read_to_string(INPUT_FILEPATH).unwrap();
    let mut monkeys: Vec<Monkey> = vec![];

    let mut items_by_monkey: HashMap<usize, Vec<u64>> = HashMap::new();

    for capture in monkey_regex.captures_iter(&input) {
        let group = |idx| capture.get(idx).unwrap().as_str();
        let monkey_id: usize = group(1).parse().unwrap();
        if monkey_id != monkeys.len() {
            panic!("unexpected order")
        }

        monkeys.push(Monkey {
            operation: Expression::from_string(group(3)),
            test: DivisibilityTest {
                divider: group(4).parse().unwrap(),
                monkey_id_true: group(5).parse().unwrap(),
                monkey_id_false: group(6).parse().unwrap(),
            },
        });

        items_by_monkey.insert(
            group(1).parse().unwrap(),
            group(2).split(", ").map(|s| s.parse().unwrap()).collect(),
        );
    }

    run(&monkeys, &mut items_by_monkey);
}
