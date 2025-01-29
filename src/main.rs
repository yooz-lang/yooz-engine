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
        cmd if cmd.starts_with("ADD") => {
            let parts: Vec<String> = command[2]
                .trim()
                .replace(&['(', ')'][..], " ")
                .split_whitespace()
                .map(|s| s.to_string())
                .collect();
            let file_path = format!("{}.yooz", parts[0]);
            insert_data(file_path, command, parts);
        }
        cmd if cmd.starts_with("FIND") => {
            let parts: Vec<String> = command[2]
                .trim()
                .replace(&['(', ')'][..], " ")
                .split_whitespace()
                .map(|s| s.to_string())
                .collect();
            let file_path = format!("{}.yooz", parts[0]);
            let _value = find_data(file_path, parts[1].clone());
        }
        _ => {
            eprintln!(
                "Unknown query: {:?}. Please use 'Create' , 'ADD' or 'FIND'.",
                &args[1..]
            );
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
                    println!("Found parent layer at {} and some {:?}", i, Some(i));
                    return Some(i);
                }
                depth -= 1;
            }
        }
        None
    }

    if layer == 1 {
        if let Some(po) = contents.find('(') {
            if contents.contains(&format!("+{}",parts[1].trim())) {
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
                if let Some(_) = contents.find(parts[1].as_str()) {
                    println!("you cant insert data with same key in the same layer");
                } else {
                    let new_layer = format!("\n+{}\n-{}\n", parts[1], command[3]);
                    contents.insert_str(po, &new_layer);
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

fn find_data(file_path: String, search: String) -> String {
    let contents = fs::read_to_string(&file_path).expect("Should have been able to read the file");
    if let Some(n_pos) = contents.find(&search) {
        let after_n = &contents[n_pos..];
        if let Some(dash_pos) = after_n.find('-') {
            let after_dash = &after_n[dash_pos + 1..];
            let value: String = after_dash
                .chars()
                .take_while(|c| !c.is_whitespace())
                .collect();
            return value;
        } else {
            "No '-' found after 'n'".to_string()
        }
    } else {
        "No 'n' found in the content".to_string()
    }
}
