use abi_stable::std_types::{RString, RVec, ROption};
use anyrun_plugin::*;
use std::fs::read_to_string;
use std::fs::OpenOptions;
use std::io::Write;
use ron::from_str;
use serde::Deserialize;
use chrono::prelude::*;

impl Default for Config {
    fn default() -> Self {
        Config {
            inbox_file: "~/org/inbox.org".into(),
        }
    }
}

#[derive(Deserialize)]
pub struct Config {
    inbox_file: String,
}

#[init]
pub fn init(config_dir: RString) -> Config {
    match read_to_string(format!("{}/orgtodo.ron", config_dir)) {
        Ok(contents) => from_str(&contents).unwrap(),
        Err(_) => Config::default(),
    }
}

#[info]
fn info() -> PluginInfo {
    PluginInfo {
        name: "org-capture TODO".into(),
        icon: "emacs".into(),
    }
}

#[get_matches]
fn get_matches(input: RString) -> RVec<Match> {
    vec![Match {
        title: input.clone(),
        icon: ROption::RSome("list-add".into()),
        use_pango: false,
        description: ROption::RSome("Add this TODO to your inbox.org".into()),
        id: ROption::RNone,
    }].into()
}

#[handler]
fn handler(selection: Match, config: &Config) -> HandleResult {
    let inbox_file = &config.inbox_file;
    let todo_text = selection.title;

    let mut file = OpenOptions::new().append(true).open(inbox_file)
        .expect("Unable to open file");

    let today = Local::now().format("%Y-%m-%d %a %H:%M").to_string();

    writeln!(file, "\n* TODO {}\n:PROPERTIES:\n:CREATED: [{}]\n:END:\n", todo_text, today)
        .expect("Unable to write to file");

    HandleResult::Close
}
