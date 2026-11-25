use std::fs;
use std::path::PathBuf;

use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug)]
#[command(name = "todo", about = "A tiny todo CLI that saves to a JSON file")]
struct Cli {
    #[arg(short, long, default_value = "todos.json")]
    file: PathBuf,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Todo {
    id: u64,
    text: String,
    done: bool,
    created: String,
}

fn main() {
    let cli = Cli::parse();
    let mut todos: Vec<Todo> = match fs::read_to_string(&cli.file) {
        Ok(s) if !s.trim().is_empty() => serde_json::from_str(&s).unwrap_or_default(),
        _ => Vec::new(),
    };

    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        print_todos(&todos);
        return;
    }

    match args[1].as_str() {
        "add" => {
            let text = args.get(2).cloned().unwrap_or_else(|| "no task".to_string());
            let next_id = todos.iter().map(|t| t.id).max().unwrap_or(0) + 1;
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_secs().to_string())
                .unwrap_or_else(|_| "0".to_string());
            todos.push(Todo { id: next_id, text: text.clone(), done: false, created: now });
            save(&cli.file, &todos);
            println!("added #{} {}", next_id, text);
        }
        "done" => {
            let n: u64 = args
                .get(2)
                .and_then(|s| s.parse().ok())
                .unwrap_or(1);
            if let Some(t) = todos.iter_mut().find(|t| t.id == n) {
                t.done = true;
                save(&cli.file, &todos);
                println!("done #{}", n);
            } else {
                eprintln!("no todo #{}", n);
            }
        }
        "rm" => {
            let n: u64 = args
                .get(2)
                .and_then(|s| s.parse().ok())
                .unwrap_or(1);
            let before = todos.len();
            todos.retain(|t| t.id != n);
            if todos.len() < before {
                save(&cli.file, &todos);
                println!("removed #{}", n);
            } else {
                eprintln!("no todo #{}", n);
            }
        }
        "list" => print_todos(&todos),
        _ => {
            eprintln!("usage: todo <add <text> | done <id> | rm <id> | list>");
        }
    }
}

fn print_todos(todos: &[Todo]) {
    if todos.is_empty() {
        println!("(empty)");
        return;
    }
    for t in todos {
        let mark = if t.done { "x" } else { " " };
        println!("[{}] {} ({})", mark, t.text, t.id);
    }
}

fn save(path: &PathBuf, todos: &[Todo]) {
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    if let Ok(s) = serde_json::to_string_pretty(todos) {
        let _ = fs::write(path, s);
    }
}
