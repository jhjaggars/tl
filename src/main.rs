#[macro_use]
extern crate serde_derive;

extern crate chrono;
extern crate clap;
extern crate chrono_humanize;
extern crate rand;
extern crate serde;
extern crate serde_json;

use clap::{Arg, App, SubCommand};
use chrono::prelude::*;
use chrono_humanize::HumanTime;
use rand::{thread_rng, Rng};
use std::collections::HashSet;
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
struct TodoList {
    items: Vec<TodoItem>,
}

impl TodoList {
    fn new() -> TodoList {
        TodoList { items: Vec::new() }
    }

    fn read(path: &str) -> TodoList {
        match fs::read(path) {
            Ok(contents) => {
                TodoList {
                    items: serde_json::from_str(&String::from_utf8_lossy(&contents)).unwrap()
                }
            },
            Err(_) => TodoList::new()
        }
    }

    fn write(&self, path: &str) {
        fs::write(path, serde_json::to_string_pretty(&self.items).unwrap()).unwrap();
    }

    fn add_many(&mut self, items: Vec<&str>) {
        for i in items {
            self.add(i);
        }
    }

    fn add(&mut self, description: &str) {
        let index = self.get_next_index();
        self.items.push(TodoItem::new(description, index));
    }

    fn get_next_index(&self) -> char {
        let possibles: HashSet<char> = String::from(
            "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ",
        ).chars()
            .collect();
        let indices = self.items
            .iter()
            .map(|item| item.index)
            .collect::<HashSet<char>>();
        let choices: Vec<char> = possibles
            .difference(&indices)
            .cloned()
            .collect::<Vec<char>>();
        let mut rng = thread_rng();
        let choice: char = *rng.choose(&choices).unwrap();
        return choice;
    }

    fn show(self) {
        for i in self.items {
            println!("{} {} created {}", i.index, i.description, HumanTime::from(i.created));
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct TodoItem {
    index: char,
    description: String,
    done: bool,
    created: DateTime<Utc>,
    updated: DateTime<Utc>,
}

impl TodoItem {
    fn new(description: &str, index: char) -> TodoItem {
        let now = Utc::now();
        TodoItem {
            index: index,
            description: description.to_string(),
            done: false,
            created: now,
            updated: now,
        }
    }
}


fn main() {
    let matches = App::new("tl")
        .version("0.1.0")
        .author("Jesse Jaggars <jhjaggars@gmail.com>")
        .about("todo list")
        .subcommand(
            SubCommand::with_name("show")
                .arg(Arg::with_name("by").help("sort by column (default done)"))
                .about("shows the list"),
        )
        .subcommand(
            SubCommand::with_name("add")
                .about("adds one or more items")
                .arg(Arg::with_name("item").required(true).multiple(true).last(
                    true,
                )),
        )
        .subcommand(
            SubCommand::with_name("remove")
                .about("removes one or more items")
                .arg(Arg::with_name("index").required(true).multiple(true).last(
                    true,
                )),
        )
        .subcommand(
            SubCommand::with_name("done")
                .about("toggles completion status on one or more items")
                .arg(Arg::with_name("index").required(true).multiple(true).last(
                    true,
                )),
        )
        .subcommand(SubCommand::with_name("test"))
        .get_matches();


    match matches.subcommand_name() {
        Some("show") => {
            let by = matches.value_of("by").unwrap_or("done");
            let todo_list = TodoList::read("tl.json");
            todo_list.show();
        }
        Some("add") => {
            let items: Vec<&str> = matches
                .subcommand_matches("add")
                .unwrap()
                .values_of("item")
                .unwrap()
                .collect();
            let mut todo_list = TodoList::read("tl.json");
            todo_list.add_many(items);
            todo_list.write("tl.json");
            println!("{}", serde_json::to_string_pretty(&todo_list).unwrap());
        }
        None => (),
        _ => (),
    }
}
