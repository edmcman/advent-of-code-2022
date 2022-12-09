use std::cmp::Ordering;

use itertools::Itertools;

enum Moves {
    Rock,
    Paper,
    Scissors,
}

fn cmp(m1: &Moves, m2: &Moves) -> Ordering {
    match (m1, m2) {
        (Moves::Rock, Moves::Rock) => Ordering::Equal,
        (Moves::Rock, Moves::Paper) => Ordering::Greater,
        (Moves::Rock, Moves::Scissors) => Ordering::Less,
        (Moves::Paper, Moves::Paper) => Ordering::Equal,
        (Moves::Paper, Moves::Scissors) => Ordering::Greater,
        (Moves::Paper, Moves::Rock) => Ordering::Less,
        (Moves::Scissors, Moves::Scissors) => Ordering::Equal,
        (Moves::Scissors, Moves::Rock) => Ordering::Greater,
        (Moves::Scissors, Moves::Paper) => Ordering::Less,
    }
}

fn move_score(m: &Moves) -> u32 {
    match m {
        Moves::Rock => 1,
        Moves::Paper => 2,
        Moves::Scissors => 3,
    }
}

fn char_to_move(s: &str) -> Moves {
    match s {
        "A" | "X" => Moves::Rock,
        "B" | "Y" => Moves::Paper,
        "C" | "Z" => Moves::Scissors,
        &_ => todo!(),
    }
}

fn score(t: (&str, &str)) -> u32 {
    let m1 = char_to_move(t.0);
    let m2 = char_to_move(t.1);
    score2(&m1, &m2)
}

fn score2(m1: &Moves, m2: &Moves) -> u32 {
    let m1_score = move_score(&m1);
    let m2_score = move_score(&m2);

    println!("m1 {m1_score} m2 {m2_score}");

    let result_score = match cmp(&m1, &m2) {
        Ordering::Greater => 6,
        Ordering::Equal => 3,
        Ordering::Less => 0,
    };

    m2_score + result_score
}

fn second_part(t: (&str, &str)) -> u32 {
    let m1 = char_to_move(t.0);

    let desired_result = match t.1 {
        "X" => Ordering::Less,
        "Y" => Ordering::Equal,
        "Z" => Ordering::Greater,
        &_ => todo!(),
    };

    // Find the move that results in the desired result
    let m2 = [Moves::Rock, Moves::Paper, Moves::Scissors]
        .iter()
        .find(|m2| cmp(&m1, m2) == desired_result)
        .unwrap();

    score2(&m1, m2)
}

fn main() {
    let s = std::io::stdin()
        .lines()
        .map(|l| l.unwrap().to_string())
        .map(|l| l.split_once(' ').unwrap())
        .collect::<Vec<(_, _)>>();

    println!("{:?}", s);

    let z: u32 = s.iter().map(|(s1, s2)| second_part((s1, s2))).sum();

    println!("cool {z}");
}
