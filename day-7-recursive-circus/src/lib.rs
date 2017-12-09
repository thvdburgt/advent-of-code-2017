use std::collections::HashMap;

#[derive(Debug)]
struct ProgramNode<'a> {
    weight: u32,
    children: Vec<&'a str>,
}

pub fn solve_puzzle_part_1(input: &str) -> String {
    root_program(&create_program_tree(input)).to_string()
}

pub fn solve_puzzle_part_2(input: &str) -> String {
    let tree = create_program_tree(input);
    let root = root_program(&create_program_tree(input));

    match weigh_tree(&tree, root) {
        WeighReturn::Imbalance(w) => w.to_string(),
        WeighReturn::Weight(_) => "no imbalance found".to_string(),
    }
}

fn create_program_tree(input: &str) -> HashMap<&str, ProgramNode> {
    // create HashMap tree
    let mut tree: HashMap<&str, ProgramNode> = HashMap::new();
    for (name, node) in input.lines().map(parse_line) {
        tree.insert(name, node);
    }
    tree
}

fn parse_line(line: &str) -> (&str, ProgramNode) {
    // split line into words
    let mut words = line.split_whitespace();

    // parse the name
    let name = words.next().expect("No name found");
    // parse the weight
    let weight = words
        .next()
        .expect("No weight found")
        .trim_matches(|c| c == '(' || c == ')')
        .parse::<u32>()
        .expect("Couldn't parse weight");
    // look for arrow
    let arrow = words.next();
    // if we found one get the names of the children
    let children = if arrow.is_some() {
        assert_eq!(arrow.unwrap(), "->");
        words.map(|s| s.trim_right_matches(|c| c == ',')).collect()
    } else {
        Vec::new()
    };

    // return name and node
    (name, ProgramNode { weight, children })
}

fn root_program<'a>(programs: &HashMap<&'a str, ProgramNode>) -> &'a str {
    // look for a program that is not contained in the list of children of any other node
    let bottom_program = programs.keys().find(|name| {
        !programs.values().any(|node| node.children.contains(name))
    });

    bottom_program.expect("no bottom program found")
}

#[derive(Clone, Debug)]
enum WeighReturn {
    Imbalance(u32),
    Weight(u32),
}

// check to see if all children of node weigh the same,
// if they weight the same return the total weight of node + children
// if they don't or there is an imbalance further down the tree returns the weight the incorrect
// child node should weigh to correct this.
fn weigh_tree(tree: &HashMap<&str, ProgramNode>, node: &str) -> WeighReturn {
    assert!(tree.contains_key(node));

    // if this is a root node return its weight
    if tree[node].children.is_empty() {
        return WeighReturn::Weight(tree[node].weight);
    }

    // create list of tuples of names of children and their weight
    let children_weights: Vec<_> = tree[node]
        .children
        .iter()
        .map(|name| (name, weigh_tree(tree, name)))
        .collect();

    // check if there is an imbalance in one of the childes
    match children_weights.iter().find(|&&(_, ref wr)| match *wr {
        WeighReturn::Imbalance(_) => true,
        _ => false,
    }) {
        None => (),
        Some(&(_, ref wr)) => return (*wr).clone(),
    };

    // unwrap all Weights in u32
    let children_weights: Vec<_> = children_weights
        .iter()
        .map(|&(n, ref wr)| match wr {
            &WeighReturn::Weight(w) => (n, w),
            _ => panic!(""),
        })
        .collect();

    // determine the weight a child should be
    let child_weight = children_weights
        .iter()
        .find(|&&(_, weight)| {
            children_weights
                .iter()
                .filter(|&&(_, w)| w == weight)
                .count() > 1
        })
        .unwrap()
        .1;

    // check if all children are that weight
    for (n, w) in children_weights {
        if w != child_weight {
            // we found a imbalance
            println!(
                "{} with weight {}, weighs a total of {}, expected {}",
                n,
                tree[n.clone()].weight,
                w,
                child_weight
            );
            let d: i32 = w as i32 - child_weight as i32;
            return WeighReturn::Imbalance((tree[n.clone()].weight as i32 - d) as u32);
        }
    }

    WeighReturn::Weight(
        tree[node].weight + child_weight * tree[node].children.len() as u32,
    )
}
