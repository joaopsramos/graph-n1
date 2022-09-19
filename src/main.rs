mod graph;
mod menu;
mod node;

use crate::{graph::*, menu::MenuOpt, node::*};
use colored::*;
use std::io;
use std::str::FromStr;

const GRAPH_SIZE: usize = 2;

fn main() {
    let mut graph = Graph {
        size: GRAPH_SIZE,
        nodes: Vec::new(),
    };
    // let mut graph = init_nodes();

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

fn init_nodes() -> Graph {
    let mut nodes = Vec::new();

    for i in 1..=GRAPH_SIZE {
        let mut code = String::new();
        let mut name = String::new();
        let mut local_type = String::new();

        print!("\n-----------------\n");
        println!("{}", format!("** Vértice {i}/{GRAPH_SIZE} **").blue());

        loop {
            let code: u32 = read_value("Código:", &mut code, Some("Código precisa ser um número"));
            let name: String = read_value("Nome do local:", &mut name, None);
            let local_type: String = read_value("Tipo do local:", &mut local_type, None);

            if nodes
                .iter()
                .map(|v: &Node| v.code)
                .collect::<Vec<u32>>()
                .contains(&code)
            {
                println!("{}", "Esse código já existe, tente usar outro".red());
                continue;
            }

            nodes.push(mut Node {
                id: i - 1,
                code,
                name,
                local_type,
                edges: Vec::new(),
            });

            break;
        }
    }

    Graph {
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
