mod feedback;
mod graph;
mod graph_builder;
mod menu;
mod node;

use crate::{
    feedback::Feedback,
    graph::*,
    menu::{MenuOpt, FILE_PATH},
};
use std::{fs, path::Path};

const GRAPH_SIZE: usize = 4;

fn main() {
    let mut graph = match setup_graph_menu() {
        Some(graph) => graph,
        None => {
            println!("\nEncerrando...");
            return;
        }
    };

    loop {
        menu::show_menu();

        let option = menu::read_option();

        if option == MenuOpt::Exit {
            println!("\nEncerrando...");
            break;
        }

        print!("\n");

        menu::run_option(option, &mut graph);
    }
}

fn setup_graph_menu() -> Option<Graph> {
    loop {
        let has_data = match fs::read_to_string(Path::new(FILE_PATH)) {
            Ok(data) => !data.is_empty(),
            Err(_) => false,
        };

        if !has_data {
            return Some(graph_builder::init_graph());
        }

        menu::show_menu_load_graph();

        return match menu::read_option() {
            MenuOpt::Save => match menu::load_graph() {
                None => {
                    println!("{}", Feedback::read_graph_file_error());
                    continue;
                }
                graph => graph,
            },
            MenuOpt::No => Some(graph_builder::init_graph()),
            MenuOpt::Exit => {
                break None;
            }
            _ => {
                println!("{}", Feedback::invalid_option());
                continue;
            }
        };
    }
}
