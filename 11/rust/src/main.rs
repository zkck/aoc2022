use std::collections::HashMap;
use std::fs;
use std::str::SplitWhitespace;

#[derive(Debug)]
enum Expression {
    Identifier(String),
    Constant(i32),
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

    fn evaluate(&self, context: &HashMap<String, i32>) -> i32 {
        match self {
            Expression::Identifier(ident) => context[ident],
            Expression::Constant(i) => *i,
            Expression::Addition(e1, e2) => e1.evaluate(context) + e2.evaluate(context),
            Expression::Multiplication(e1, e2) => e1.evaluate(context) * e2.evaluate(context),
        }
    }
}

trait Test {
    fn get_monkey(&self, worry_level: i32) -> usize;
}

#[derive(Debug)]
struct DivisibilityTest {
    divider: i32,
    monkey_id_true: usize,
    monkey_id_false: usize,
}

impl Test for DivisibilityTest {
    fn get_monkey(&self, worry_level: i32) -> usize {
        if worry_level % self.divider == 0 {
            self.monkey_id_true
        } else {
            self.monkey_id_false
        }
    }
}

trait Operation {
    fn change_worry_level();
}

#[derive(Debug)]
struct Monkey {
    operation: Expression,
    test: DivisibilityTest,
}

const NUM_ROUNDS: usize = 20;

const MONKEY_REGEX: &str = r"Monkey (\d+):
  Starting items: (.+)
  Operation: new = (.+)
  Test: divisible by (\d+)
    If true: throw to monkey (\d)
    If false: throw to monkey (\d)
";

fn run(monkeys: &HashMap<usize, Monkey>, items_by_monkey: &mut HashMap<usize, Vec<i32>>) {
    let mut inspection_counts: HashMap<usize, usize> = HashMap::new();

    for _round in 0..NUM_ROUNDS {
        for monkey_id in 0..8 {
            let monkey = monkeys.get(&monkey_id).expect("monkey should be in map");
            let items = match items_by_monkey.remove(&monkey_id) {
                Some(v) => v,
                None => vec![],
            };

            *inspection_counts.entry(monkey_id).or_default() += items.len();

            for mut item_worry_level in items {
                let mut context = HashMap::new();
                context.insert("old".to_string(), item_worry_level);

                item_worry_level = monkey.operation.evaluate(&context);
                item_worry_level /= 3;

                let to_monkey_id = monkey.test.get_monkey(item_worry_level);
                let entry = items_by_monkey.entry(to_monkey_id);
                entry.or_insert(vec![]).push(item_worry_level);
            }
        }

        println!("{:?}", items_by_monkey);
    }

    let mut inspection_values: Vec<&usize> = inspection_counts.values().collect();
    inspection_values.sort();
    inspection_values.reverse();

    println!("{:?}", inspection_counts);
    println!("ans1 {:?}", inspection_values.iter().take(2).fold(1, |a, &b| {a * b}));
}

fn main() {
    let monkey_regex = regex::Regex::new(MONKEY_REGEX).unwrap();

    let input = fs::read_to_string("../input").unwrap();
    let mut monkeys: HashMap<usize, Monkey> = HashMap::new();

    let mut items_by_monkey: HashMap<usize, Vec<i32>> = HashMap::new();

    for capture in monkey_regex.captures_iter(&input) {
        let group = |idx| capture.get(idx).unwrap().as_str();
        monkeys.insert(
            group(1).parse().unwrap(),
            Monkey {
                operation: Expression::from_string(group(3)),
                test: DivisibilityTest {
                    divider: group(4).parse().unwrap(),
                    monkey_id_true: group(5).parse().unwrap(),
                    monkey_id_false: group(6).parse().unwrap(),
                },
            },
        );

        items_by_monkey.insert(
            group(1).parse().unwrap(),
            group(2).split(", ").map(|s| s.parse().unwrap()).collect(),
        );
    }

    run(&monkeys, &mut items_by_monkey);
}
