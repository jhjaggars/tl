use chrono::prelude::*;
use chrono_humanize::HumanTime;
use colored::*;
use rand::{thread_rng, Rng};
use std::collections::{HashSet, HashMap};
use std::fs;
use std::iter::FromIterator;

#[derive(Serialize, Deserialize, Debug)]
pub struct TodoList {
    items: HashMap<char, TodoItem>,
}

impl TodoList {
    fn new() -> TodoList {
        TodoList { items: HashMap::new() }
    }

    pub fn read(path: &str) -> TodoList {
        match fs::read(path) {
            Ok(contents) => {
                TodoList {
                    items: serde_json::from_str(&String::from_utf8_lossy(&contents)).unwrap(),
                }
            }
            Err(_) => TodoList::new(),
        }
    }

    pub fn write(&self, path: &str) {
        fs::write(path, serde_json::to_string_pretty(&self.items).unwrap()).unwrap();
    }

    pub fn add_many(&mut self, items: &Vec<String>) {
        for i in items {
            self.add(i.to_string());
        }
    }

    fn add(&mut self, description: String) {
        let index = self.get_next_index();
        self.items.insert(index, TodoItem::new(&description, index));
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
        let item = self.items.remove(&index);
        match item {
            None => println!("Couldn't find an item at index '{}' to remove.", index),
            Some(i) => println!("Ok. Removing item {} '{}'", index, i.description),
        }
    }

    pub fn remove_many(&mut self, items: Vec<char>) {
        for i in items {
            self.remove(i);
        }
    }

    pub fn done_many(&mut self, items: Vec<char>) {
        for ch in items {
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
