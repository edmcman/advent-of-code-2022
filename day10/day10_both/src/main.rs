#[derive(Debug)]
struct State {
    cycle: u32,
    x: i32,
}

enum Instruction {
    Noop,
    Addx(i32),
}

impl Instruction {
    fn from_string(s: &str) -> Self {
        match (s.split_once(' '), s) {
            (None, "noop") => Self::Noop,
            (Some(("addx", i)), _) => Self::Addx(i.parse().expect("Expected integer")),
            _ => panic!("unknown insn")
        }
    }

    fn run(&self, st: &State) -> Vec<State> {
        match self {
            Self::Noop => Vec::from([State {
                cycle: st.cycle + 1,
                ..*st
            }]),
            Self::Addx(i) => Vec::from([
            State {
                cycle: st.cycle+1,
                ..*st
            },
            State {
                cycle: st.cycle + 2,
                x: st.x + i,
            }]),
        }
    }
}

fn main() {
    let insns: Vec<Instruction> = std::io::stdin()
        .lines()
        .map(|l| l.unwrap())
        .map(|s| Instruction::from_string(&s))
        .collect();

    let init_state = State{cycle: 0, x: 1};

    let sts = insns.iter().fold(Vec::from([init_state]), |mut sts, i| { sts.extend(i.run(sts.iter().last().unwrap())); sts});

    println!("states: {:?}", sts);

    let indices = [20, 60, 100, 140, 180, 220];

    let signal_strengths = indices.map(|i: usize| i32::try_from(i).unwrap()*sts.get(i-1).unwrap().x);

    println!("signal strengths: {:?} sum: {}", signal_strengths, signal_strengths.iter().sum::<i32>());

    let rows = 0..=5;
    let cycles: Vec<Vec<usize>> = rows.map(|r| ((40*r)..(40*r + 40)).collect::<Vec<_>>()).collect();

    println!("cycles: {:?}", cycles);

    let print_stuff = cycles.iter().map(|v| v.iter().map(|i| {
        let st = sts.get(*i).unwrap();
        let sprite_x = st.x;
        let cycle_x = i32::try_from(i%40).unwrap();
        let sprite_range = [sprite_x-1, sprite_x, sprite_x+1];
        let visible = sprite_range.iter().find(|i| **i == cycle_x).is_some();
        if visible { '#' } else { '.' }
    }).collect::<String>());

    print_stuff.for_each(|v| println!("{v}"));
}
