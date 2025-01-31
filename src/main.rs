use std::{
    env,
    fs::{self, File},
    io::Write,
};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        print_help();
        return;
    }
    let command: Vec<String> = parse_query(&args[1]);
    match args[1].as_str() {
        "help" | "-h" | "--help" => print_help(),
        "--version" | "-v" => {
            println!("yooz version 0.1.0");
            return;
        }
        "create" | "CREATE" => {
            let mut file = match fs::File::create(format!("{}.yooz", &args[2])) {
                Err(why) => panic!("couldn't create db: {}", why),
                Ok(file) => file,
            };
            if let Err(why) = write_to_db(&mut file) {
                panic!("couldn't insert data to : {}.yooz", why);
            }
            println!("{} created successfully", args[2])
        }
        cmd if cmd.starts_with("add to") || cmd.starts_with("ADD TO") => {
            let parts: Vec<String> = command[2]
                .trim()
                .replace(&['(', ')'][..], " ")
                .split_whitespace()
                .map(|s| s.to_string())
                .collect();
            let file_path = format!("{}.yooz", parts[0]);
            insert_data(
                file_path,
                parts[1].clone(),
                parts[2].parse().unwrap(),
                command[3].clone(),
            );
        }
        cmd if cmd.starts_with("find in") || cmd.starts_with("FIND IN") => {
            let parts: Vec<String> = key_layer_value_parts(&command[2]);
            let file_path = format!("{}.yooz", parts[0]);
            let value = find_data(file_path, parts[1].clone(), parts[2].parse().unwrap());
            println!("{} is {:?}", parts[1], value.0);
        }
        cmd if cmd.starts_with("remove from") || cmd.starts_with("REMOVE FROM") => {
            let parts: Vec<String> = key_layer_value_parts(&command[2]);
            let file_path = format!("{}.yooz", parts[0]);
            remove_data(file_path, parts[1].clone(), parts[2].parse().unwrap())
        }
        cmd if cmd.starts_with("change from") || cmd.starts_with("CHANGE from") => {
            let parts: Vec<String> = key_layer_value_parts(&command[2]);
            let file_path = format!("{}.yooz", parts[0]);
            update_data(
                file_path,
                parts[1].clone(),
                parts[2].parse().unwrap(),
                command[3].clone(),
            );
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
fn key_layer_value_parts(query: &String) -> Vec<String> {
    query
        .trim()
        .replace(&['(', ')'][..], " ")
        .split_whitespace()
        .map(|s| s.to_string())
        .collect()
}
fn parse_query(query: &String) -> Vec<String> {
    query
        .trim()
        .split_whitespace()
        .map(|s| s.to_string())
        .collect()
}

fn remove_data(file_path: String, key: String, layer: usize) {
    let contents = fs::read_to_string(&file_path).expect("Should have been able to read the file");
    let (_, position) = find_data(file_path.clone(), key.clone(), layer);
    if position == usize::MAX {
        println!("{} not found in layer {}", key, layer);
        return;
    }
    let mut end = position - 1;
    let mut newline_count = 0;
    for (i, c) in contents[position - 1..].chars().enumerate() {
        if c == '\n' {
            newline_count += 1;
        }
        if newline_count == 2 {
            end = position + i;
            break;
        }
    }
    let mut updated_contents = contents.clone();
    updated_contents.replace_range(position - 1..=end, "");

    fs::write(&file_path, updated_contents).expect("Unable to write file");
    println!("{}({}) deleted successfully", key, layer);
}
fn insert_data(file_path: String, key: String, layer: usize, value: String) {
    let mut contents = fs::read_to_string(&file_path).unwrap();
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
            if contents.contains(&format!("+{}", key.trim())) {
                println!("you cant insert data with same key in the same layer");
            } else {
                let new_data = format!("\n+{}\n-{}\n", key, value);
                contents.insert_str(po + 1, &new_data);
            }
        }
    } else {
        if let Some(parent_pos) = find_parent_layer(&contents, layer - 1) {
            if let Some(po) = contents[..parent_pos].rfind(")") {
                if let Some(_) = contents[..po].rfind("(") {
                    if contents.contains(&format!("+{}", key.trim())) {
                        println!("you cant insert data with same key in the same layer");
                    } else {
                        let new_layer = format!("\n+{}\n-{}\n", key, value);
                        contents.insert_str(po, &new_layer);
                    }
                }
            } else {
                let new_layer = format!("\n(\n\n+{}\n-{}\n\n)\n", key, value);
                contents.insert_str(parent_pos, &new_layer);
            }
        } else {
            println!(
                "you cant insert in layer {} when layer {} does not exist",
                layer,
                layer - 1
            );
            return;
        }
    }
    let mut file = File::create(&file_path).expect("create failed");
    file.write_all(contents.as_bytes()).expect("write failed");
}

fn update_data(file_path: String, key: String, layer: usize, new_value: String) {
    let contents = fs::read_to_string(&file_path).expect("Should have been able to read the file");
    let (_, position) = find_data(file_path.clone(), key.clone(), layer);
    if position == usize::MAX {
        println!("{} not found in layer {}", key, layer);
        return;
    }
    let mut end = position;
    let mut newline_count = 0;
    for (i, c) in contents[position..].chars().enumerate() {
        if c == '\n' {
            newline_count += 1;
        }
        if newline_count == 2 {
            end = position + i;
            break;
        }
    }
    let mut updated_contents = contents.clone();
    if let Some(dash_pos) = contents[position..end].find('-') {
        let value_start = position + dash_pos + 1;
        let value_end = contents[value_start..]
            .chars()
            .position(|c| c == '\n' || c == ')')
            .map(|rel_end| value_start + rel_end)
            .unwrap_or(end);

        updated_contents.replace_range(value_start..value_end, &new_value);

        fs::write(&file_path, updated_contents).expect("Unable to write to file");
        println!("{}({}) updated successfully", key, layer);
    } else {
        println!("No valid value found in the specified key");
    }
}
fn find_data(file_path: String, search: String, layer: usize) -> (String, usize) {
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
                        return (value, n_pos_offset);
                    } else {
                        println!("Value of {} not found", search);
                    }
                }
            }
        }
        i += 1;
    }
    (
        format!("{} does not exist in layer {}", search, layer).to_string(),
        usize::MAX,
    )
}

fn print_help() {
    println!(
        r#"
Yooz Database Engine 

Usage:
    yooz "<command>"

Commands:
    CREATE     Create a database. Usage: CREATE dbname
    ADD     Add data to the database. Usage: "ADD TO dbname(key(layer)) value"
    FIND     Find data in the database. Usage: "FIND IN dbname(key(layer))"
    CHANGE     Change specific data from the database. Usage: "CHANGE FROM dbname(key(layer)) new-value"
    REMOVE     Remove specific data from the database. Usage: "REMOVE FROM dbname(key(layer))"
    help    Show this help message.

Examples:
    yooz CREATE mydb
    yooz "ADD To mydb(name(1)) mohammad"
    yooz "FIND IN mydb(name(1))"


For more details, you can check the documentation in http://yooz.run/.
"#
    );
}
