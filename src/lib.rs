use abi_stable::std_types::{RString, RVec, ROption};
use anyrun_plugin::*;
use std::fs::OpenOptions;
use std::io::Write;
use chrono::prelude::*;

#[init]
fn init(_config_dir: RString) {
    // Initialization code if needed
}

#[info]
fn info() -> PluginInfo {
    PluginInfo {
        name: "TODO".into(),
        icon: "list-add".into(), // Icon from the icon theme
    }
}

#[get_matches]
fn get_matches(input: RString) -> RVec<Match> {
    vec![Match {
        title: input.clone(),
        icon: ROption::RSome("list-add".into()),
        use_pango: false,
        description: ROption::RSome("Add this TODO to your inbox".into()),
        id: ROption::RNone,
    }].into()
}

#[handler]
fn handler(selection: Match) -> HandleResult {
    let inbox_file = "/path/to/your/inbox.org";
    let todo_text = selection.title;

    let mut file = OpenOptions::new().append(true).open(inbox_file)
        .expect("Unable to open file");

    let today = Local::now().format("%Y-%m-%d %a %H:%M").to_string();

    writeln!(file, "\n* TODO {}\n:PROPERTIES:\n:CREATED: [{}]\n:END:\n", todo_text, today)
        .expect("Unable to write to file");

    HandleResult::Close
}
