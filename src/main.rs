#![feature(extern_prelude)]
extern crate chrono;
extern crate chrono_humanize;
extern crate clap;
extern crate colored;
extern crate rand;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use clap::{Arg, App, SubCommand, ArgMatches};

mod todo;

fn _get_all<'a>(matches: &'a ArgMatches<'a>, subc: &str, argn: &str) -> Vec<&'a str> {
    matches
    .subcommand_matches(subc)
    .unwrap()
    .values_of(argn)
    .unwrap()
    .collect()
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
        .get_matches();


    match matches.subcommand_name() {
        Some("show") => {
            let by = matches.value_of("by").unwrap_or("done");
            let todo_list = todo::TodoList::read("tl.json");
            todo_list.show(by);
        }
        Some("add") => {
            let items = _get_all(&matches, "add", "item");
            let mut todo_list = todo::TodoList::read("tl.json");
            todo_list.add_many(&items);
            todo_list.write("tl.json");
            println!("Ok, added {} items.", items.len());
        }
        Some("remove") => {
            let items = _get_all(&matches, "remove", "index");
            let mut todo_list = todo::TodoList::read("tl.json");
            todo_list.remove_many(&items);
            todo_list.write("tl.json");
        }
        Some("done") => {
            let items = _get_all(&matches, "done", "index");
            let mut todo_list = todo::TodoList::read("tl.json");
            todo_list.done_many(&items);
            todo_list.write("tl.json");
        }
        None => {
            let by = matches.value_of("by").unwrap_or("done");
            let todo_list = todo::TodoList::read("tl.json");
            todo_list.show(by);
        },
        _ => (),
    }
}
