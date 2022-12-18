use parse_display::FromStr;
use pathfinding::directed::dijkstra;
use std::collections::HashMap;

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

#[derive(Clone, Debug, PartialEq, Eq, Hash, Copy)]
enum Who {
    Me,
    Elephant,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct State {
    who: Who,
    at_valves: (String, String),
    mins: u32,
    open_valves: Vec<String>
    //pressure: u32,
}

impl State {
    /*
    fn hash(&self) -> (Who, (String, String), u32, Vec<String>) {
        (&self.who, &self.at_valves, &self.mins, &self.open_valves)
    }*/

    fn get_valve(&self) -> &String {
        match self.who {
            Who::Me => &self.at_valves.0,
            Who::Elephant => &self.at_valves.1,
        }
    }

    fn moveme(&self, s: &str) -> Self {
        let c = self.clone();
        Self {
            at_valves: match self.who {
                Who::Me => (s.to_string(), c.at_valves.1),
                Who::Elephant => (c.at_valves.0, s.to_string()),
            },
            ..c
        }
    }

    fn next_turn(&self, valves: &HashMap<String, Valve>) -> Self {
        let (who, mins, update_pressure) = match (self.who, self.mins) {
            (Who::Me, mins) => (Who::Elephant, mins, false),
            (Who::Elephant, mins) => (Who::Me, mins - 1, true),
        };
        Self {
            who,
            mins,
            /*pressure: if (update_pressure) {
                self.pressure
                    + self
                        .open_valves
                        .iter()
                        .map(|ov| valves.get(ov).unwrap().flow)
                        .sum::<u32>()
            } else {
                self.pressure
            },*/
            ..self.clone()
        }
    }

    fn new() -> Self {
        Self {
            mins: 26,
            open_valves: Vec::new(),
            at_valves: ("AA".to_string(), "AA".to_string()),
            who: Who::Me,
            //pressure: 0,
        }
    }
}

type Sequence = (Vec<Action>, u32);

/*
fn search(valves: &HashMap<String,Valve>, st: State) {
    let mut best = st.clone();
    let mut wl: Vec<State> = vec![st];


    while let Some(current) = wl.pop() {
        let score = current.pressure;
        if score > best.pressure {
            best = current.clone();
            println!("Best: {} {}", best.pressure, best.mins);
        };
        let mut new_moves: Vec<State> = moves(&current, &valves).into_iter().map(|(a,b)| b).collect();
        wl.append(&mut new_moves);
    }
}*/

fn moves(st: &State, valves: &HashMap<String, Valve>) -> Vec<(Action, State)> {
    let at_valve = valves.get(st.get_valve()).expect("valid valve");

    // Hey, we can always do nothing!
    let mut next_moves = vec![];

    // With 2 minutes, we can open the current valve
    if st.mins >= 2 && !st.open_valves.contains(&at_valve.name) && at_valve.flow > 0 {
        let action = Action::Open(at_valve.name.to_string());
        let mut new_valves = st.open_valves.clone();
        new_valves.push(at_valve.name.to_string());
        new_valves.sort();

        let new_state = State {
            open_valves: new_valves,
            //pressure: st.pressure + at_valve.flow * (st.mins - 1),
            ..st.next_turn(&valves)
        };

        next_moves.push((action, new_state));
    };

    // With 3 minutes, we can immediately walk somewhere else and open a valve.
    if st.mins >= 3 {
        at_valve.valves.iter().for_each(|adjacent_valve| {
            let action = Action::Walk(adjacent_valve.to_string());
            let new_state = st.moveme(adjacent_valve).next_turn(&valves);

            next_moves.push((action, new_state));
        });
    };

    next_moves
}

fn best(
    st: &State,
    valves: &HashMap<String, Valve>,
    best_memo: &mut HashMap<State, Sequence>,
) -> Option<Sequence> {

    let memo = best_memo.get(st);
    let memo = memo.or_else(|| match st.who {
        Who::Me => {
            let c = st.clone();
            let reversed_st = State {
                at_valves: match c.at_valves {
                    (a, b) => (b, a),
                },
                ..c
            };
            best_memo.get(&reversed_st)
        }
        _ => None,
    });
    match memo {
        Some(x) => Some((*x).clone()),
        None => {
            let at_valve = valves.get(st.get_valve()).expect("valid valve");

            // Hey, we can always do nothing!
            //let moves = moves(&st, valves);

            let mut possible_actions = vec![];

            // With 2 minutes, we can open the current valve
            if st.mins >= 2 && !st.open_valves.contains(&at_valve.name) && at_valve.flow > 0 {
                let mut new_actions = vec![Action::Open(at_valve.name.to_string())];
                let mut new_valves = st.open_valves.clone();
                let new_score = at_valve.flow * (st.mins - 1);
                new_valves.push(at_valve.name.to_string());
                new_valves.sort();
                if new_valves.len() > 10 {
                    println!("hmm {}", new_valves.len())
                }

                let new_state = State {
                    open_valves: new_valves,
                    ..st.next_turn(&valves)
                };
                let (mut best_actions, best_score) = best(&new_state, valves, best_memo, best_mins);
                new_actions.append(&mut best_actions);
                let new_score = new_score + best_score;

                possible_actions.push((new_actions, new_score));
            };

            // With 3 minutes, we can immediately walk somewhere else and open a valve.
            if st.mins >= 3 {
                at_valve.valves.iter().for_each(|adjacent_valve| {
                    let mut new_actions = vec![Action::Walk(adjacent_valve.to_string())];
                    let new_state = st.moveme(adjacent_valve).next_turn(&valves);
                    let (mut best_actions, best_score) = best(&new_state, valves, best_memo, best_mins);
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

            /*if st.mins > 16 {
                dbg!(st.mins);
            }*/

            Some(best_move)
        }
    }

    //possible_actions
}

fn djikstra_helper<'a>(s: &'a String, valves: &'a HashMap<String, Valve>) -> HashMap<String, i32> {

    dbg!(s);

    let object = dijkstra::dijkstra_all(s, |v| {
        valves
            .get(v)
            .expect("djikstra")
            .valves
            .iter()
            .map(|valve| (valve.to_string(), 1))
    })
    .into_iter()
    .filter(|(k,_)| valves.get(k).unwrap().flow > 0)
    .map(|(k, v)| (k, v.1))
    .collect();

    //dbg!(&object);
    object
}

fn main() {
    let valves: HashMap<String, Valve> = std::io::stdin()
        .lines()
        .map(|l| l.unwrap())
        .map(|s| s.parse().expect("parse"))
        .map(|vh: Valve| (vh.name.clone(), vh))
        .collect();

    dbg!(&valves);

    let mut valves_with_flow_and_start: Vec<&String> = valves
        .iter()
        .filter_map(|(n, v)| if v.flow > 0 { Some(n) } else { None })
        .collect();
    valves_with_flow_and_start.push(&valves.get("AA").unwrap().name);

    let dists: HashMap<(String, String), i32> = valves_with_flow_and_start
        .iter()
        .flat_map(|src| {
            let d_results = djikstra_helper(*src, &valves);
            dbg!(&d_results);
            d_results
                .into_iter()
                .map(|(dst, dist)| ((src.to_string(), dst), dist))
        })
        .collect();

    dbg!(&dists);

    let st = State::new();

}
