#[macro_use]
extern crate serde_derive;

extern crate chrono;
extern crate clap;
extern crate chrono_humanize;
extern crate colored;
extern crate rand;
extern crate serde;
extern crate serde_json;

use clap::{Arg, App, SubCommand};
use chrono::prelude::*;
use chrono_humanize::HumanTime;
use colored::*;
use rand::{thread_rng, Rng};
use std::collections::{HashSet, HashMap};
use std::fs;
use std::iter::FromIterator;

#[derive(Serialize, Deserialize, Debug)]
struct TodoList {
    items: HashMap<char, TodoItem>,
}

impl TodoList {
    fn new() -> TodoList {
        TodoList { items: HashMap::new() }
    }

    fn read(path: &str) -> TodoList {
        match fs::read(path) {
            Ok(contents) => {
                TodoList {
                    items: serde_json::from_str(&String::from_utf8_lossy(&contents)).unwrap(),
                }
            }
            Err(_) => TodoList::new(),
        }
    }

    fn write(&self, path: &str) {
        fs::write(path, serde_json::to_string_pretty(&self.items).unwrap()).unwrap();
    }

    fn add_many(&mut self, items: &Vec<&str>) {
        for i in items {
            self.add(i);
        }
    }

    fn add(&mut self, description: &str) {
        let index = self.get_next_index();
        self.items.insert(index, TodoItem::new(description, index));
    }

    fn get_next_index(&self) -> char {
        let possibles: HashSet<char> = String::from(
            "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ",
        ).chars()
            .collect();
        let indices = self.items.keys().cloned().collect();
        let choices: Vec<char> = possibles
            .difference(&indices)
            .cloned()
            .collect::<Vec<char>>();
        if choices.len() < 1 {
            panic!(
                "No more item indices remain!  You need to remove some things before adding more!"
            );
        }
        let mut rng = thread_rng();
        let choice: char = *rng.choose(&choices).unwrap();
        return choice;
    }

    fn show(&self, by: &str) {
        let mut items = Vec::from_iter(self.items.values());
        items.sort_by_key(|item| item.attr(by));
        for i in items {
            println!(
                "{} {} {} {}",
                i.index.to_string().cyan(),
                i.get_done(),
                i.description,
                HumanTime::from(i.created).to_string().black().bold()
            );
        }
    }

    fn remove(&mut self, index: &str) {
        let item = self.items.remove(&index.chars().next().unwrap());
        match item {
            None => println!("Couldn't find an item at index '{}' to remove.", index),
            Some(i) => println!("Ok. Removing item {} '{}'", index, i.description),
        }
    }

    fn remove_many(&mut self, items: &Vec<&str>) {
        for i in items {
            self.remove(i);
        }
    }

    fn done_many(&mut self, items: &Vec<&str>) {
        for i in items {
            let ch = i.chars().next().unwrap();
            let item = self.items.get_mut(&ch);
            match item {
                Some(el) => el.done = !el.done,
                None => println!("No item at index '{}'.", ch),
            }
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

    fn attr(&self, key: &str) -> String {
        match key {
            "index" => self.index.to_string(),
            "done" => self.done.to_string(),
            "created" => self.created.to_string(),
            "updated" => self.updated.to_string(),
            _ => self.created.to_string(),
        }
    }

    fn get_done(&self) -> colored::ColoredString {
        match self.done {
            true => "\u{2611}".green().bold(),
            false => "\u{2610}".white(),
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
            todo_list.show(by);
        }
        Some("add") => {
            let items: Vec<&str> = matches
                .subcommand_matches("add")
                .unwrap()
                .values_of("item")
                .unwrap()
                .collect();
            let mut todo_list = TodoList::read("tl.json");
            todo_list.add_many(&items);
            todo_list.write("tl.json");
            println!("Ok, added {} items.", items.len());
        }
        Some("remove") => {
            let items: Vec<&str> = matches
                .subcommand_matches("remove")
                .unwrap()
                .values_of("index")
                .unwrap()
                .collect();
            let mut todo_list = TodoList::read("tl.json");
            todo_list.remove_many(&items);
            todo_list.write("tl.json");
        }
        Some("done") => {
            let items: Vec<&str> = matches
                .subcommand_matches("done")
                .unwrap()
                .values_of("index")
                .unwrap()
                .collect();
            let mut todo_list = TodoList::read("tl.json");
            todo_list.done_many(&items);
            todo_list.write("tl.json");
        }
        None => (),
        _ => (),
    }
}
