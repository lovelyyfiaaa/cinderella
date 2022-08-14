use cinderella::{app::App, init_terminal, confirm::ConfirmArgsBuilder};
use clap::{arg, Command};
use std::{path::PathBuf, process::exit};

fn cli() -> Command<'static> {
    Command::new("clinderella")
        .about("Make beautiful CLI apps that are as beautiful as Cinderella!!! 💖✨✨")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .allow_invalid_utf8_for_external_subcommands(true)
        .subcommand(
            Command::new("input")
                .about("Ask the user for a string input!!")
                .arg(arg!(<TEXT> "The text to provide the user with"))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("confirm")
                .about("Ask the user whether we should continue with doing an option.")
                .after_help("Returns with exit code 1 if the user doesn't want it, and 0 if the user wants to proceed.")
                .args(&[
                    arg!([prompt] "The text to provide the user with"),
                    arg!(--affirmative [affirmative] "The title of the affirmative action"),
                    arg!(--negative [negative] "The title of the negative action"),
                    arg!(--default "Use the default action.")
                    ])
               
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("add")
                .about("adds things")
                .arg_required_else_help(true)
                .arg(arg!(<PATH> ... "Stuff to add").value_parser(clap::value_parser!(PathBuf))),
        )
}

fn main() {
    let matches = cli().get_matches();
    let mut terminal = init_terminal().unwrap();
    let mut app = App::new();
    let mut return_with = 0;
    match matches.subcommand() {
        Some(("confirm", _sub_matches)) => {
            let prompt: Option<String> = matches
                .get_one("[prompt]")
                .map(|str: &String| str.to_owned());

            let negative: Option<String> = matches
                .get_one("[negative]")
                .map(|str: &String| str.to_owned());
             
            let affirmative: Option<String> = matches
                .get_one("[affirmative]")
                .map(|str: &String| str.to_owned());
            loop {
                let prompt = prompt.clone();
                let negative = negative.clone();
                let affirmative = affirmative.clone();

                terminal.draw(|f| {
                    app.confirm_render(f, None,
                    "confirm", 
                    ConfirmArgsBuilder::default()
                            .prompt(prompt)
                            .negative(negative)
                            .affirmative(affirmative)
                            .build().unwrap());
                           

                }).unwrap();

                if let Some(result) = app.confirm_event("confirm") {
           
                    exit(if result { 0 } else { 1 });
                 
                };
        }
        }
        None => unreachable!(),
        _ => unimplemented!(),
    }

    exit(return_with);
}
