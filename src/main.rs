mod graph;
mod menu;
mod node;

use crate::{
    graph::*,
    menu::{Feedback, MenuOpt, FILE_PATH},
    node::*,
};
use colored::*;
use std::{fs, io, path::Path, str::FromStr};

const GRAPH_SIZE: usize = 4;

fn main() {
    let mut graph = match setup_graph_menu() {
        Some(graph) => graph,
        None => {
            println!("Encerrando...");
            return;
        }
    };

    loop {
        menu::show_menu();

        let option = menu::read_option();

        if option == MenuOpt::Exit {
            println!("Encerrando...");
            break;
        }

        print!("\n");

        menu::run_option(option, &mut graph);
    }

    // dbg!(graph);
}

fn setup_graph_menu() -> Option<Graph> {
    loop {
        let has_data = match fs::read_to_string(Path::new(FILE_PATH)) {
            Ok(data) => !data.is_empty(),
            Err(_) => false,
        };

        if !has_data {
            return Some(init_nodes());
        }

        menu::show_menu_load_graph();
        return match menu::read_option() {
            MenuOpt::Save => match menu::load_graph() {
                None => {
                    println!("{}", Feedback::read_graph_file_error());
                    continue;
                }
                graph => return graph,
            },
            MenuOpt::No => Some(init_nodes()),
            MenuOpt::Exit => {
                println!("Encerrando...");
                break None;
            }
            _ => {
                println!("{}", Feedback::invalid_option());
                continue;
            }
        };
    }
}

fn init_nodes() -> Graph {
    let mut nodes = Vec::new();

    let size = format!("{}", GRAPH_SIZE.to_string().green());
    let text = format!("\nMonte seu grafo com {size} vértices").cyan();
    println!("{}", text.bold().italic());

    for i in 1..=GRAPH_SIZE {
        let mut name = String::new();
        let mut local_type = String::new();

        print!("\n-----------------\n");
        println!("{}", format!("** Vértice {i}/{GRAPH_SIZE} **").blue());

        loop {
            let name: String = read_value("Nome do local:", &mut name, None);
            let local_type: String = read_value("Tipo do local:", &mut local_type, None);

            if nodes
                .iter()
                .map(|v: &Node| v.code)
                .collect::<Vec<usize>>()
                .contains(&i)
            {
                println!("{}", "Esse código já existe, tente usar outro".red());
                continue;
            }

            nodes.push(Node {
                code: i,
                name,
                local_type,
                edges: Vec::new(),
            });

            break;
        }
    }

    Graph {
        is_weighted: false,
        size: GRAPH_SIZE,
        nodes,
    }
}

fn read_value<T>(text: &str, value: &mut String, error_msg: Option<&str>) -> T
where
    T: FromStr,
{
    let error_msg = error_msg.unwrap_or("Erro ao ler valor, por favor, digite novamente...");

    loop {
        *value = String::new();

        println!("{}", text.yellow());

        io::stdin().read_line(value).unwrap();

        match value.trim().parse() {
            Ok(parsed_value) => return parsed_value,
            Err(_) => {
                println!("{}", error_msg.red());
                continue;
            }
        };
    }
}
