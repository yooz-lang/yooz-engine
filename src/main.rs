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
            print!("{:?}", parts);
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
            let value = find_data(file_path, parts[1].clone());
            print!("{}", value);
        }
        _ => {
            eprintln!("Unknown command: {:?}. Please use 'a' or 'b'.", command);
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
    let mut contents =
        fs::read_to_string(&file_path).expect("Should have been able to read the file");

    if let Some(pos) = contents.find(')') {
        let new_data = format!("+{}\n-{}\n\n", parts[1], &command[3]);
        contents.insert_str(pos, &new_data);
    }
    print!("{}", contents);
    let mut file = File::create(&file_path).expect("create failed");
    let _ = file.write_all(contents.as_bytes());
}

fn find_data(file_path: String, search: String) -> String {
    let contents =
    fs::read_to_string(&file_path).expect("Should have been able to read the file");
      if let Some(n_pos) = contents.find(&search) {
        let after_n = &contents[n_pos..];
        if let Some(dash_pos) = after_n.find('-') {
            let after_dash = &after_n[dash_pos + 1..];
            let value: String = after_dash
                .chars()
                .take_while(|c| !c.is_whitespace()) 
                .collect();
            return value
        } else {
             "No '-' found after 'n'".to_string()
        }
    } else {
         "No 'n' found in the content".to_string()
    }
}