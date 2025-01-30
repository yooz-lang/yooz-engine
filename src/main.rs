use std::{
    env,
    fs::{self, File},
    io::Write,
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let command: Vec<String> = parse_query(&args[1]);
    match args[1].as_str() {
        "Create" => {
            let mut file = match fs::File::create(format!("{}.yooz", &args[2])) {
                Err(why) => panic!("couldn't create db: {}", why),
                Ok(file) => file,
            };
            if let Err(why) = write_to_db(&mut file) {
                panic!("couldn't write to file: {}", why);
            }
        }
        cmd if cmd.starts_with("add to") || cmd.starts_with("ADD TO") => {
            let parts: Vec<String> = command[2]
                .trim()
                .replace(&['(', ')'][..], " ")
                .split_whitespace()
                .map(|s| s.to_string())
                .collect();
            let file_path = format!("{}.yooz", parts[0]);
            insert_data(file_path, command, parts);
        }
        cmd if cmd.starts_with("find in") || cmd.starts_with("FIND IN") => {
            let parts: Vec<String> = command[2]
                .trim()
                .replace(&['(', ')'][..], " ")
                .split_whitespace()
                .map(|s| s.to_string())
                .collect();
            let file_path = format!("{}.yooz", parts[0]);
            let value = find_data(file_path, parts[1].clone(), parts[2].parse().unwrap());
            println!("{}", value);
        }
        cmd if cmd.starts_with("remove from") || cmd.starts_with("REMOVE FROM") => {
            let parts: Vec<String> = command[2]
                .trim()
                .replace(&['(', ')'][..], " ")
                .split_whitespace()
                .map(|s| s.to_string())
                .collect();
            let file_path = format!("{}.yooz", parts[0]);
            remove_data(file_path, parts[1].clone())
        }
        _ => {
            eprintln!("Unknown query: {}.", &args[1..].join(" "));
        }
    }
}
fn write_to_db(file_name: &mut fs::File) -> std::io::Result<()> {
    writeln!(file_name, "(\n\n\n)\n")?;
    Ok(())
}
fn parse_query(query: &String) -> Vec<String> {
    query
        .trim()
        .split_whitespace()
        .map(|s| s.to_string())
        .collect()
}

fn remove_data(_file_path: String, _key: String) {}
fn insert_data(file_path: String, command: Vec<String>, parts: Vec<String>) {
    let mut contents = fs::read_to_string(&file_path).unwrap();
    let layer: usize = parts[2].parse().expect("Layer must be a positive integer");
    if layer == 0 {
        println!("Error: Layer 0 is not valid.");
        return;
    }
    fn find_parent_layer(contents: &str, parent_layer: usize) -> Option<usize> {
        let mut depth = 0;
        for (i, c) in contents.chars().enumerate() {
            if c == '(' {
                depth += 1;
            } else if c == ')' {
                if depth == parent_layer {
                    return Some(i);
                }
                depth -= 1;
            }
        }
        None
    }
    if layer == 1 {
        if let Some(po) = contents.find('(') {
            if contents.contains(&format!("+{}", parts[1].trim())) {
                println!("you cant insert data with same key in the same layer");
            } else {
                let new_data = format!("\n+{}\n-{}\n", parts[1], command[3]);
                contents.insert_str(po + 1, &new_data);
            }
        }
    } else {
        if let Some(parent_pos) = find_parent_layer(&contents, layer - 1) {
            println!("pos {}", parent_pos);
            if let Some(po) = contents[..parent_pos].rfind(")") {
                if let Some(_) = contents[..po].rfind("(") {
                    if contents.contains(&format!("+{}", parts[1].trim())) {
                        println!("you cant insert data with same key in the same layer");
                    } else {
                        let new_layer = format!("\n+{}\n-{}\n", parts[1], command[3]);
                        contents.insert_str(po, &new_layer);
                    }
                }
            } else {
                let new_layer = format!("(\n+{}\n-{}\n)\n", parts[1], command[3]);
                contents.insert_str(parent_pos, &new_layer);
            }
        } else {
            println!("Error: layer {} does not exist.", layer - 1,);
            return;
        }
    }
    let mut file = File::create(&file_path).expect("create failed");
    file.write_all(contents.as_bytes()).expect("write failed");
}

fn _print_char_positions(contents: &str) {
    for (index, character) in contents.chars().enumerate() {
        println!("Index: {}, Character: '{}'", index, character);
    }
}
fn _find_layer(contents: &str, layer: usize) -> Option<usize> {
    let mut depth = 0;
    for (i, c) in contents.chars().enumerate() {
        if c == '(' {
            depth += 1;
            if depth == layer {
                println!("{}", i);
                return Some(i);
            }
        } else if c == ')' {
            depth -= 1
        }
    }
    None
}

fn find_data(file_path: String, search: String, layer: usize) -> String {
    println!("layer {} search {}", layer, search);
    let contents = fs::read_to_string(&file_path).expect("Should have been able to read the file");

    let mut depth = 0;
    let mut i = 0;

    while i < contents.len() {
        let c = contents.chars().nth(i).unwrap();

        match c {
            '(' => {
                depth += 1;
            }
            ')' => {
                depth -= 1;
            }
            _ => {
                if depth == layer && contents[i..].starts_with(&search) {
                    let n_pos_offset = i;
                    let after_n = &contents[n_pos_offset..];

                    if let Some(dash_pos) = after_n.find('-') {
                        let after_dash = &after_n[dash_pos + 1..];
                        let value: String = after_dash
                            .chars()
                            .take_while(|c| !c.is_whitespace() && *c != ')')
                            .collect();

                        return value; 
                    } else {
                        println!("Value of {} not found", search);
                    }
                }
            }
        }

        i += 1;
    }

    format!("{} does not exist in layer {}", search,layer).to_string()
}