use std::io;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn show_todo_list() -> String {
    let path = "D:\\w\\To-Do_list_rust\\To-Do_files\\todo.txt";
    let text = fs::read_to_string(path).expect("None");
    println!("With text:\n{text}");
    text
}

fn count_lines_in_file() -> io::Result<usize> {
    let path = "D:\\w\\To-Do_list_rust\\To-Do_files\\todo.txt";
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let line_count = reader.lines().count();
    Ok(line_count)
}

fn add_new_task(new_todo:String) -> bool {
    let path = "D:\\w\\To-Do_list_rust\\To-Do_files\\todo.txt";
    let count_line = match count_lines_in_file() {
        Ok(count) => count,
        Err(e) => {
            println!("Error reading file: {}", e);
            0
        }
    };

    let mut file = OpenOptions::new()
    .append(true)
    .create(true)
    .open(path)
    .expect("error");

    let mut new_line = count_line;
    new_line += 1;

    let line_text = format!("\n{}. {}", new_line, new_todo);
    file.write_all(line_text.as_bytes()).expect("error");
    println!("Task added!");
    true
}

fn edit_todo_list(line_number: usize, new_text: &str) -> bool {
    let path = "D:\\w\\To-Do_list_rust\\To-Do_files\\todo.txt";
    let contents = fs::read_to_string(path);
    let mut lines: Vec<String> = contents.expect("failed").lines().map(|s| s.to_string()).collect();

    if line_number == 0 || line_number > lines.len() {
        println!("Line number out of range.");
        return false;
    }

    lines[line_number - 1] = format!("{}. {}", line_number, new_text.trim());

    let new_contents = lines.join("\n");
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(path)
        .expect("Failed to open file");

    file.write_all(new_contents.as_bytes()).expect("Failed to write to file");
    println!("✅ Line {} updated.", line_number);
    true
}

fn delete_todo_list() {
    let path = "D:\\w\\To-Do_list_rust\\To-Do_files\\todo.txt";
    fs::remove_file(path).expect("could not remove file");
    File::create(path).expect("could not recreate file");
    println!("Removed file data.txt")
}

fn main() {
    let program_run = false;  
    println!("Please enter your selection.");
    println!("1.(show my list)\n2.(add new task)\n3.(edit To-Do list)\n4.(delete To-Do list)\n5.(exit)");

    while program_run != true {
        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).expect("Unable to read Stdin");

        if user_input.trim() == "1" {
            show_todo_list();
            println!("Please enter your selection.");
            println!("1.(show my list)\n2.(add new task)\n3.(edit To-Do list)\n4.(delete To-Do list)5.(exit)");
        }else if user_input.trim() == "2" {
            println!("Please Enter your new Task");
            let mut new_todo = String::new();
            io::stdin().read_line(&mut new_todo).expect("");
            let result = add_new_task(new_todo);
            if result == false {
                println!("error")
            }
            println!("Please enter your selection.");
            println!("1.(show my list)\n2.(add new task)\n3.(edit To-Do list)\n4.(delete To-Do list)5.(exit)");
        }else if user_input.trim() == "3" {
            show_todo_list();
            let mut edit_line = String::new();
            io::stdin().read_line(&mut edit_line).expect("Failed to read line");

            let line_number: usize = match edit_line.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("❗ Invalid line number!");
                    return;
                }
            };
            println!("Please Enter your Task");
            let mut new_task = String::new();
            io::stdin().read_line(&mut new_task).expect("");
            edit_todo_list(line_number,new_task.trim());
            show_todo_list();

            println!("Please enter your selection.");
            println!("1.(show my list)\n2.(add new task)\n3.(edit To-Do list)\n4.(delete To-Do list)5.(exit)");
        }else if user_input.trim() == "4" {
         delete_todo_list();

        }else if user_input.trim() == "5"{
            break;
        }else{
            println!("Please Enter corroct input!");
            continue;
        }
    }
}