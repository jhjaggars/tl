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

    pub fn read(path: &str) -> TodoList {
        match fs::read(path) {
            Ok(contents) => {
                let _items = serde_json::from_str(&String::from_utf8_lossy(&contents))
                    .expect("Data file failed to parse! Is it corrupt?");
                TodoList {
                    items: _items,
                    file: path.to_string(),
                }
            }
            Err(_) => TodoList::new(),
        }
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
        let c = self.get_next_index().unwrap();
        self.items
            .insert(c, TodoItem::new(&description.trim_right(), c));
    }

    fn get_next_index(&self) -> Result<char, &'static str> {
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
            return Err(
                "No more item indices remain!  You need to remove some things before adding more!",
            );
        }
        let mut rng = thread_rng();
        let choice: char = *rng.choose(&choices).unwrap();
        return Ok(choice);
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

    pub fn done_many(&mut self, items: Vec<char>) {
        for ch in items {
            let item = self.items.get_mut(&ch);
            match item {
                Some(el) => el.done = !el.done,
                None => println!("No item at index '{}'.", ch),
            }
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
            true => "\u{2713}".green().bold(),
            false => {
                let now = Utc::now();
                let elapsed = now.signed_duration_since(self.created);
                let glyph = "\u{25cf}";
                if elapsed.num_days() > 1 {
                    glyph.red()
                } else if elapsed.num_hours() > 4 {
                    glyph.yellow()
                } else {
                    glyph.white()
                }
            }
        }
    }
}
