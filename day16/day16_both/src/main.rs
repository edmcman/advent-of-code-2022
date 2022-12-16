use parse_display::FromStr;
use std::collections::{HashMap, HashSet};

#[derive(FromStr, PartialEq, Debug)]
#[display("Valve {name} has flow rate={flow}; tunnel lead to valves {valves}")]
#[from_str(new = Self::new(name, flow, valves), regex="Valve (?P<name>.+) has flow rate=(?P<flow>.+); tunnels? leads? to valves? (?P<valves>.+)")]
struct Valve {
    name: String,
    flow: u32,
    valves: Vec<String>,
}

impl Valve {
    fn new(name: String, flow: u32, valves: String) -> Self {
        Valve {
            name: String::from(name),
            flow,
            valves: valves.split(", ").map(|s| String::from(s)).collect(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum Action {
    Walk(String),
    Open(String),
    Nothing,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct State {
    at_valve: String,
    mins: u32,
    open_valves: Vec<String>,
}

impl State {
    fn new() -> Self {
        Self {
            mins: 30,
            open_valves: Vec::new(),
            at_valve: "AA".to_string()
        }
    }
}

type Sequence = (Vec<Action>, u32);

fn best(
    st: &State,
    valves: &HashMap<String, Valve>,
    best_memo: &mut HashMap<State, Sequence>,
) -> Sequence {
    let memo = best_memo.get(st);
    match memo {
        Some(x) => (*x).clone(),
        None => {

            let at_valve = valves.get(&st.at_valve).expect("valid valve");

            // Hey, we can always do nothing!
            let mut possible_actions = vec![(vec![Action::Nothing], 0)];

            // With 2 minutes, we can open the current valve
            if st.mins >= 2 && !st.open_valves.contains(&at_valve.name) && at_valve.flow > 0 {
                let mut new_actions = vec![Action::Open(at_valve.name.to_string())];
                let mut new_valves = st.open_valves.clone();
                let new_score = at_valve.flow * (st.mins - 1);
                new_valves.push(at_valve.name.to_string());

                let new_state = State {
                    mins: st.mins - 1,
                    open_valves: new_valves,
                    at_valve: at_valve.name.to_string()
                };
                let (mut best_actions, best_score) = best(&new_state, valves, best_memo);
                new_actions.append(&mut best_actions);
                let new_score = new_score + best_score;

                possible_actions.push((new_actions, new_score));
            };

            // With 3 minutes, we can immediately walk somewhere else and open a valve.
            if st.mins >= 3 {
                at_valve.valves.iter().for_each(|adjacent_valve| {
                    let mut new_actions = vec![Action::Walk(adjacent_valve.to_string())];
                    let new_state = State {
                        mins: st.mins - 1,
                        at_valve: adjacent_valve.to_string(),
                        ..st.clone()
                    };
                    let (mut best_actions, best_score) = best(&new_state, valves, best_memo);
                    new_actions.append(&mut best_actions);

                    possible_actions.push((new_actions, best_score));
                });
            };

            //dbg!(&possible_actions);

            let best_move = possible_actions
                .into_iter()
                .max_by_key(|(acts, score)| *score)
                .expect("always have an action");

            best_memo.insert(st.clone(), best_move.clone());

            best_move
        }
    }

    //possible_actions
}

fn main() {
    let valves: HashMap<String, Valve> = std::io::stdin()
        .lines()
        .map(|l| l.unwrap())
        .map(|s| s.parse().expect("parse"))
        .map(|vh: Valve| (vh.name.clone(), vh))
        .collect();

    dbg!(&valves);

    let mut best_memo: HashMap<State, Sequence> = HashMap::new();

    let st = State::new();
    let b = best(&st, &valves, &mut best_memo);

    println!("best: {:?}", b)
}
