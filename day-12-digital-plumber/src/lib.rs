use std::collections::{HashMap, HashSet};

pub fn solve_puzzle_part_1(input: &str) -> String {
    let programs: HashMap<usize, HashSet<usize>> = input.lines().map(to_program).collect();

    depth_first_search(&programs, 0).len().to_string()
}

pub fn solve_puzzle_part_2(input: &str) -> String {
    let programs: HashMap<usize, HashSet<usize>> = input.lines().map(to_program).collect();
    let groups = program_groups(&programs);

    groups.len().to_string()
}

fn to_program(s: &str) -> (usize, HashSet<usize>) {
    let mut words = s.split_whitespace();
    let program = words.next().unwrap().parse::<usize>().unwrap();
    words.next();
    let pipes = words
        .map(|word| word.trim_right_matches(',').parse::<usize>().unwrap())
        .collect();

    (program, pipes)
}

fn program_groups(programs: &HashMap<usize, HashSet<usize>>) -> Vec<HashSet<usize>> {
    let mut groups: Vec<HashSet<usize>> = Vec::new();

    for node in programs.keys() {
        let mut not_seen_before = true;
        for group in &groups {
            if group.contains(node) {
                not_seen_before = false;
                break;
            }
        }
        if not_seen_before {
            let group = depth_first_search(programs, *node);
            groups.push(group);
        }
    }

    groups
}

fn depth_first_search(programs: &HashMap<usize, HashSet<usize>>, node: usize) -> HashSet<usize> {
    fn dfs_rec(
        programs: &HashMap<usize, HashSet<usize>>,
        node: usize,
        visited: &mut HashSet<usize>,
    ) {
        visited.insert(node);

        for neighbour in programs.get(&node).unwrap() {
            if !visited.contains(neighbour) {
                dfs_rec(programs, *neighbour, visited);
            }
        }
    }

    let mut visited: HashSet<usize> = HashSet::new();
    dfs_rec(programs, node, &mut visited);

    visited
}
