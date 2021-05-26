use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;

fn main() {
    if env::args().len() < 2 {
        println!("usage: parser_utils <grammar-file>");
        std::process::exit(0);
    }
    let args: Vec<String> = env::args().collect();
    let contents = fs::read_to_string(&args[1]).expect("File not found!");

    let mut terminals: HashSet<String> = HashSet::new();
    let mut productions: HashMap<String, Vec<Vec<String>>> = HashMap::new();
    let mut first_sets: HashMap<String, HashSet<String>> = HashMap::new();
    let mut follow_sets: HashMap<String, HashSet<String>> = HashMap::new();
    let mut start_symbol = String::new();

    // process the file
    for line in contents.split('\n') {
        let parts: Vec<&str> = line.split(' ').collect();
        if parts.len() > 0 && parts[0].len() > 0 {
            if parts[0] == "terminals" {
                for i in 1..parts.len() {
                    terminals.insert(String::from(parts[i]));
                }
            } else {
                if start_symbol.len() == 0 {
                    let old_start_symbol = String::from(parts[0]);
                    start_symbol = String::from(&format!("__{}", parts[0]));
                    productions.insert(start_symbol.clone(), vec![vec![old_start_symbol.clone()]]);
                }
                if !productions.contains_key(parts[0]) {
                    productions.insert(String::from(parts[0]), Vec::new());
                }
                let mut prod: Vec<String> = Vec::new();
                for i in 1..parts.len() {
                    prod.push(String::from(parts[i]));
                }
                productions.get_mut(parts[0]).unwrap().push(prod);
            }
        }
    }
    println!("Terminals: {:?}", terminals);
    println!("Grammar Start Symbol: {}", start_symbol);
    println!("Productions");
    for (head, prods) in &productions {
        print!("{}", head);
        for prod in prods {
            println!("\t{:?}", prod);
        }
        println!();
    }

    // init first
    for (head, _) in &productions {
        first_sets.insert(String::from(head), HashSet::new());
    }
    // calc first sets
    loop {
        let mut changed = false;
        for (head, prods) in &productions {
            for body in prods {
                let first_symbol = &body[0];
                // A is terminal
                if terminals.contains(first_symbol) {
                    if !first_sets[head].contains(first_symbol) {
                        changed = true;
                        first_sets
                            .get_mut(head)
                            .unwrap()
                            .insert(first_symbol.to_string());
                    }
                }
                // if A -> EMPTY
                else if first_symbol == "~" {
                    first_sets
                        .get_mut(head)
                        .unwrap()
                        .insert(first_symbol.to_string());
                    changed = true;
                }
                // if A is a nonterminal
                else {
                    let new_symbols = first_sets[first_symbol].clone();
                    for symbol in new_symbols {
                        if !first_sets[head].contains(&symbol) {
                            first_sets.get_mut(head).unwrap().insert(symbol);
                            changed = true;
                        }
                    }
                }
            }
        }
        if !changed {
            break;
        }
    }
    println!("First Sets");
    for (key, value) in &first_sets {
        println!("{} => {:?}", key, value);
    }

    // init follow
    for (head, _) in &productions {
        follow_sets.insert(String::from(head), HashSet::new());
    }
    // calc followsets
    follow_sets
        .get_mut(&start_symbol)
        .unwrap()
        .insert(String::from("$"));
    loop {
        let mut changed = false;
        for (head, prods) in &productions {
            for prod in prods {
                for i in 0..prod.len() {
                    // nonterminal followed by terminal
                    if i < prod.len() - 1
                        && !terminals.contains(&prod[i])
                        && terminals.contains(&prod[i + 1])
                        && !follow_sets[&prod[i]].contains(&prod[i + 1])
                    {
                        follow_sets
                            .get_mut(&prod[i])
                            .unwrap()
                            .insert(prod[i + 1].clone());
                        changed = true;
                    }
                    // alone nonterminal
                    else if &prod[i] != "~"
                        && !terminals.contains(&prod[i])
                        && (i == prod.len() - 1)
                    {
                        let symbols = follow_sets[head].clone();
                        for symbol in symbols {
                            if !follow_sets[&prod[i]].contains(&symbol) {
                                follow_sets
                                    .get_mut(&prod[i])
                                    .unwrap()
                                    .insert(symbol.clone());
                                changed = true;
                            }
                        }
                    }
                    // nonterminal followed by nonterminal
                    else if (i < prod.len() - 1)
                        && (&prod[i] != "~" && &prod[i + 1] != "~")
                        && (!terminals.contains(&prod[i]) && !terminals.contains(&prod[i + 1]))
                    {
                        let symbols = first_sets[&prod[i + 1]].clone();
                        for symbol in symbols {
                            if !follow_sets[&prod[i]].contains(&symbol) {
                                follow_sets
                                    .get_mut(&prod[i])
                                    .unwrap()
                                    .insert(symbol.clone());
                                changed = true;
                            }
                        }
                    }
                }
            }
        }
        if !changed {
            break;
        }
    }
    println!("Follow Sets");
    for (key, value) in &follow_sets {
        println!("{} => {:?}", key, value);
    }
}
