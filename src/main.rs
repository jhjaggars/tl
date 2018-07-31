extern crate chrono;
extern crate chrono_humanize;
extern crate colored;
extern crate docopt;
extern crate rand;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use docopt::Docopt;

mod todo;

const USAGE: &'static str = "
tl

Usage:
  tl [options]
  tl [options] add <task>...
  tl [options] done <index>...
  tl [options] remove <index>...
  tl (-h | --help)
  tl --version

Options:
  --by=<by>            Sort by value.
  -f FILE --file=FILE  Todo list file [default: ./todo.json].
  -h --help            Show this screen.
  --version            Show version.
";

#[derive(Debug, Deserialize)]
struct Args {
    flag_by: String,
    arg_task: Vec<String>,
    arg_index: Vec<char>,
    flag_file: String,
    cmd_add: bool,
    cmd_done: bool,
    cmd_remove: bool,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    let mut todo_list = todo::TodoList::read(&args.flag_file);

    if args.cmd_add {
        todo_list.add_many(&args.arg_task);
    } else if args.cmd_done {
        todo_list.done_many(args.arg_index);
    } else if args.cmd_remove {
        todo_list.remove_many(args.arg_index);
    } else {
        todo_list.show(&args.flag_by);
    }
}
