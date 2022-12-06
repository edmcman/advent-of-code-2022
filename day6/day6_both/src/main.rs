use std::collections::HashSet;

fn find_marker(window: usize, s: &str) -> Option<usize> {
    let c: Vec<char> = s.chars().collect();
    c.windows(window)
        .enumerate()
        .find(|(index, w)| w.iter().collect::<HashSet<_>>().len() == window)
        .map(|t| t.0)
        .map(|i| i+window)
}

fn main() {
    let line = std::io::stdin().lines().map(|l| l.unwrap()).next().unwrap();
    println!("found 4 marker at {:?}", find_marker(4, &line));
    println!("found 14 marker at {:?}", find_marker(14, &line));
}
