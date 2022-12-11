use itertools::Itertools;
use num::Integer;
use std::cell::RefCell;

type Worry = usize;

#[derive(Debug)]
enum Operand {
    Old,
    Const(Worry),
}

impl Operand {
    fn from_string(s: &str) -> Self {
        match s {
            "old" => Self::Old,
            i => Self::Const(i.parse().unwrap()),
        }
    }

    fn compute(&self, old: Worry) -> Worry {
        match self {
            Self::Old => old,
            Self::Const(n) => *n,
        }
    }
}

#[derive(Debug)]
enum Operation {
    Plus(Operand, Operand),
    Times(Operand, Operand),
}

impl Operation {
    fn from_string(s: &str) -> Self {
        match s[2..].split(" ").collect::<Vec<_>>().as_slice() {
            ["Operation:", "new", "=", op1, op, op2] => {
                let op1 = Operand::from_string(op1);
                let op2 = Operand::from_string(op2);
                match *op {
                    "*" => Self::Times(op1, op2),
                    "+" => Self::Plus(op1, op2),
                    _ => panic!("what op?"),
                }
            }
            _ => panic!("bad operation"),
        }
    }

    fn compute(&self, old: Worry) -> Worry {
        match self {
            Self::Plus(op1, op2) => op1.compute(old) + op2.compute(old),
            Self::Times(op1, op2) => op1.compute(old) * op2.compute(old),
        }
    }
}

#[derive(Debug)]
enum Test {
    DivisibleBy(Worry),
}

impl Test {
    fn from_string(s: &str) -> Self {
        match s[2..].split(' ').collect::<Vec<_>>().as_slice() {
            ["Test:", "divisible", "by", n] => Self::DivisibleBy(n.parse().unwrap()),
            _ => panic!("divisible :("),
        }
    }

    fn compute(&self, worry: Worry) -> bool {
        match self {
            Self::DivisibleBy(d) => worry.is_multiple_of(d),
        }
    }
}

#[derive(Debug)]
struct Next {
    true_dest: usize,
    false_dest: usize,
}

impl Next {
    fn from_strings(strs: &[String]) -> Self {
        match strs
            .iter()
            .map(|s| &s[4..])
            .map(|s| dbg!(s).split(' ').nth(5).unwrap().parse::<usize>().unwrap())
            .take(2)
            .collect::<Vec<usize>>()
            .as_slice()
        {
            [t, f] => Self {
                true_dest: *t,
                false_dest: *f,
            },
            _ => panic!("uh oh"),
        }
    }
}

#[derive(Debug)]
struct Monkey {
    worry: Vec<Worry>,
    op: Operation,
    t: Test,
    n: Next,
    num_inspections: usize,
}

impl Monkey {
    fn from_strings(strs: &[String]) -> Self {
        dbg!(strs);
        /*
        let id: usize = match dbg!(strs.get(0).unwrap().split_once(' ')) {
            Some(("Monkey", i)) => i.parse().unwrap(),
            _ => panic!("whoops"),
        };*/

        let starting: Vec<_> = match strs.get(1).unwrap()[2..]
            .split(' ')
            .collect::<Vec<_>>()
            .as_slice()
        {
            ["Starting", "items:", items @ ..] => items
                .iter()
                .flat_map(|s| s.chars())
                //.inspect(|c| { dbg!(*c); })
                .collect::<String>()
                .split(',')
                .map(|s| dbg!(s).parse::<Worry>().unwrap())
                .collect(),
            l => {
                dbg!(l);
                panic!("whoops")
            }
        };

        let op = Operation::from_string(&strs[2]);

        let t = Test::from_string(&strs[3]);

        let n = Next::from_strings(&strs[4..=5]);

        Monkey {
            worry: starting,
            op,
            t,
            n,
            num_inspections: 0,
        }
    }
}

fn main() {
    let i = std::io::stdin().lines().map(|l| l.unwrap());
    let l: Vec<_> = i.collect();

    let monkeys: Vec<_> = l
        .chunks(7)
        .map(|strs| Monkey::from_strings(strs))
        .map(|m| RefCell::new(m))
        .collect();

    let lcm = monkeys
        .iter()
        .map(|m| match m.borrow().t {
            Test::DivisibleBy(n) => n,
        })
        .reduce(|a, b| a.lcm(&b))
        .unwrap();

    println!("lcm: {lcm}");

    for i in 1..=10000 {
        //dbg!((i, &monkeys));

        for mr in monkeys.iter() {
            let mut n = 0;
            // Make sure the reference goes out of scope.
            {
                let m = mr.borrow_mut();
                let new_dests = m.worry.iter().map(|item| {
                    let old_inspect = item;
                    n += 1;
                    let new_worry_intermediate = m.op.compute(*old_inspect);
                    let new_worry = new_worry_intermediate % lcm;
                    //let new_worry = new_worry_intermediate.div_floor(&3);
                    //println!("old {old_inspect} inter {new_worry_intermediate} new {new_worry}");
                    let t = m.t.compute(new_worry);
                    let dest = if t { m.n.true_dest } else { m.n.false_dest };
                    (new_worry, dest)
                });

                new_dests.for_each(|(i, m)| {
                    monkeys.get(m).unwrap().borrow_mut().worry.push(i);
                });
            }

            mr.borrow_mut().num_inspections += n;

            mr.borrow_mut().worry.clear()
        }
    }

    println!("Monkeys {:?}", monkeys);
    let most_inspections: usize = monkeys
        .iter()
        .map(|m| m.borrow().num_inspections)
        .sorted()
        .rev()
        .take(2)
        .product();
    println!("Ok {most_inspections}");
}
