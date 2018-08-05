use chrono::prelude::*;
use chrono_humanize::HumanTime;
use colored;
use colored::Colorize;
use rand::{thread_rng, Rng};
use serde_json;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::io::{self, BufRead};
use std::iter::FromIterator;

const CHECKMARK: &'static str = "\u{2713}";
const CIRCLE: &'static str = "\u{25cf}";

#[derive(Serialize, Deserialize, Debug)]
pub struct TodoList {
    items: HashMap<char, TodoItem>,
    file: String,
}

impl TodoList {
    fn new() -> TodoList {
        TodoList {
            items: HashMap::new(),
            file: "./todo.json".to_string(),
        }
    }

    pub fn read(path: &str) -> TodoList{
        fs::read(path).and_then(|contents| {
            let _items = serde_json::from_str(&String::from_utf8_lossy(&contents))
                .expect("Data file failed to parse! Is it corrupt?");
            Ok(TodoList {
                items: _items,
                file: path.to_string(),
            })
        }).unwrap_or_else(|_e| TodoList::new())
    }

    pub fn write(&self) {
        let data = serde_json::to_string(&self.items).expect("Failed to serialize data!");
        fs::write(&self.file, data).expect("Failed to save data!");
    }

    pub fn add_many(&mut self, items: &Vec<String>) {
        for i in items {
            if i == "-" {
                let stdin = io::stdin();
                let mut handle = stdin.lock();
                handle.lines().for_each(|line| self.add(line.unwrap()));
            } else {
                self.add(i.to_string());
            }
        }
        self.write();
    }

    fn add(&mut self, description: String) {
        self.get_next_index().and_then(|c| {
            self.items
                .insert(c, TodoItem::new(&description.trim_right(), c))
        });
    }

    fn get_used(&self) -> HashSet<char> {
        self.items.keys().cloned().collect()
    }

    fn get_next_index(&self) -> Option<char> {
        let possibles = HashSet::from_iter(
            "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars(),
        );
        let choices: Vec<char> = possibles.difference(&self.get_used()).cloned().collect();
        let mut rng = thread_rng();
        rng.choose(&choices).cloned()
    }

    pub fn show(&self, by: &str) {
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

    fn remove(&mut self, index: char) {
        match self.items.remove(&index) {
            None => println!("Couldn't find an item at index '{}' to remove.", index),
            Some(i) => println!("Ok. Removing item {} '{}'", index, i.description),
        }
    }

    pub fn remove_many(&mut self, items: Vec<char>) {
        items.iter().for_each(|i| self.remove(*i));
        self.write();
    }

    fn done(&mut self, c: char) {
        match self.items.get_mut(&c) {
            Some(el) => el.done = !el.done,
            None => println!("No item at index '{}'.", c),
        }
    }

    pub fn done_many(&mut self, items: Vec<char>) {
        for ch in items {
            self.done(ch);
        }
        self.write();
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
            true => CHECKMARK.green().bold(),
            false => {
                let now = Utc::now();
                let elapsed = now.signed_duration_since(self.created);
                if elapsed.num_days() > 1 {
                    CIRCLE.red()
                } else if elapsed.num_hours() > 4 {
                    CIRCLE.yellow()
                } else {
                    CIRCLE.white()
                }
            }
        }
    }
}
