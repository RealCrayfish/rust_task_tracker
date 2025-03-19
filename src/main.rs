// Making a task tracker app.
use colored::Colorize;
use serde_json::Value;
use std::fs;
use std::fs::File;
use std::path::Path;
use uuid::{Timestamp, Uuid};

const TASK_FILE_PATH: &str = "tasks.json";

fn main() {
    println!("Task Tracker writen in rust!");

    let mut quit = false;
    while !quit {
        println!("Select an option:");
        println!("1: List Tasks\n2: Add Task\n3: Delete Task\n4: Quit");

        // Read input, and cast to string literal.
        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice).unwrap();
        let choice: &str = choice.trim();

        // Werid ahh switch() syntax.
        match choice {
            "1" => list_tasks(),
            "2" => add_task(),
            "3" => delete_task(),
            "4" => quit = true,
            _ => continue,
        }
    }

    println!("Did something");
}

fn list_tasks() {
    create_taskfile();

    // Parse the json into something I can use.
    let mut file_contents = String::new();
    file_contents = fs::read_to_string(TASK_FILE_PATH).expect("Unable to read file");
    let v: Value =
        serde_json::from_str(file_contents.trim()).expect("end of file or something idk");

    // Display.
    if let Some(tasks) = v["tasks"].as_array() {
        for task in tasks {
            println!("{}", task["id"].as_str().unwrap());
            println!("{}", task["title"].as_str().unwrap().bold());
            println!("{}", task["description"].as_str().unwrap().italic());
            println!("");
        }
    }
}

fn add_task() {
    create_taskfile();

    println!("Enter task title:");

    let mut title = String::new();
    std::io::stdin()
        .read_line(&mut title)
        .expect("Failed to read line");

    println!("Enter task description:");

    let mut description = String::new();
    std::io::stdin()
        .read_line(&mut description)
        .expect("Failed to read line");

    let task = serde_json::json!({
        "id": Uuid::new_v4().to_string(),
        "title": title.trim(),
        "description": description.trim()
    });

    let mut file_contents = String::new();
    file_contents = fs::read_to_string(TASK_FILE_PATH).expect("Unable to read file");
    let v: Value =
        serde_json::from_str(file_contents.trim()).expect("end of file or something idk");

    let mut tasks = v["tasks"].as_array().unwrap().to_vec();
    tasks.push(task);

    let mut file = File::create(TASK_FILE_PATH).expect("cannot create file");
    serde_json::to_writer_pretty(&mut file, &serde_json::json!({"tasks": tasks}))
        .expect("cannot write to file");
}

fn delete_task() {
    create_taskfile();

    let mut file_contents = String::new();
    file_contents = fs::read_to_string(TASK_FILE_PATH).expect("Unable to read file");
    let v: Value =
        serde_json::from_str(file_contents.trim()).expect("end of file or something idk");

    let mut task_id = String::new();
    println!("Enter task ID:");
    std::io::stdin()
        .read_line(&mut task_id)
        .expect("Failed to read line");
    let task_id = task_id.trim().to_string();
    let mut tasks = v["tasks"].as_array().unwrap().to_vec();

    tasks.retain(|task| task["id"].as_str().unwrap() != task_id);

    let mut file = File::create(TASK_FILE_PATH).expect("cannot create file");
    serde_json::to_writer_pretty(&mut file, &serde_json::json!({"tasks": tasks}))
        .expect("cannot write to file");
}

fn create_taskfile() {
    // Creates a new task file if one does not exist in the cwd.
    // TODO: Change to use a data path rather than the cwd.
    if !Path::new(TASK_FILE_PATH).is_file() {
        let mut file = File::create(TASK_FILE_PATH).expect("cannot create file");
    }
}
