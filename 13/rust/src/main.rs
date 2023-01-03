use std::io;

// #[derive(Debug)]
// enum List<V> {
//     Cons(V, Box<List<V>>),
//     Nil
// }

#[derive(Debug, Clone)]
enum Value {
    Integer(u32),
    List(Vec<Value>),
}

type ComparisonResult = Option<bool>;

impl Value {
    fn from_chars<I>(input: &mut I) -> (Option<Self>, Option<char>)
    where
        I: Iterator<Item = char>,
    {
        match input.next().expect("expected token") {
            '[' => {
                // A vector begins
                let mut vector = vec![];
                loop {
                    let (maybe_item, stoptoken) = Value::from_chars(input);
                    if let Some(item) = maybe_item {
                        vector.push(item);
                    }
                    match stoptoken {
                        Some(']') => break,
                        Some(',') => {
                            // continue
                        }
                        _ => panic!("list not closed"),
                    }
                }
                (Some(Self::List(vector)), input.next())
            }
            mut c => {
                let mut acc = String::new();
                while c.is_numeric() {
                    acc.push(c);
                    // we can always expect char, since the top level is always a list
                    c = input.next().unwrap();
                }
                let result = if acc.is_empty() {
                    None
                } else {
                    Some(Self::Integer(acc.parse().unwrap()))
                };
                (result, Some(c))
            }
        }
    }
}

fn less_than(first: &Value, second: &Value) -> ComparisonResult {
    match (first, second) {
        (Value::Integer(a), Value::Integer(b)) => {
            if a == b {
                None
            } else {
                Some(a < b)
            }
        }
        (Value::Integer(a), Value::List(_)) => {
            less_than(&Value::List(vec![Value::Integer(*a)]), second)
        }
        (Value::List(_), Value::Integer(b)) => {
            less_than(first, &Value::List(vec![Value::Integer(*b)]))
        }
        (Value::List(a), Value::List(b)) => {
            if a.is_empty() && b.is_empty() {
                None
            } else if a.is_empty() {
                // only a is empty
                Some(true)
            } else if b.is_empty() {
                // only b is empty
                Some(false)
            } else {
                // compare heads
                match less_than(a.first().unwrap(), b.first().unwrap()) {
                    Some(b) => Some(b),
                    None => less_than(&Value::List(a[1..].to_vec()), &Value::List(b[1..].to_vec())),
                }
            }
        }
    }
}

fn main() -> io::Result<()> {
    let lines: Vec<String> = io::stdin().lines().map(|l| l.unwrap()).filter(|l| !l.is_empty()).collect();
    let mut sum = 0;
    for (i, slice) in lines.chunks(2).enumerate() {
        if let [a, b] = slice {
            let (a, _) = Value::from_chars(&mut a.chars());
            let (b, _) = Value::from_chars(&mut b.chars());
            println!("{:?}", a);
            println!("{:?}", b);
            if less_than(&a.unwrap(), &b.unwrap()).unwrap() {
                sum += i + 1
            }
        }
    }
    println!("{}", sum);
    Ok(())
}
