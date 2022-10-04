use crate::{
    feedback::*,
    graph::Graph,
    node::{Edge, Node},
};
use colored::Colorize;
use std::{fs, io, path::Path};
use MenuOpt::*;

type RunOptResult = Result<String, String>;

pub const FILE_PATH: &str = "./graph.json";

#[derive(PartialEq)]
pub enum MenuOpt {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    No,
    Load,
    Visualize,
    Save,
    Export,
    Exit,
}

pub fn show_menu_load_graph() {
    println!(
        "\n{}",
        "-------------------------------------------------------------------------------".magenta()
    );
    println!("{}", "** Menu **".blue().bold());
    println!(
        "{}",
        "Detectamos um grafo existente, desja carrega-lo?".yellow()
    );

    println!(
        "\
{}) Sim
{}) Não
{}) Encerrar",
        "s".magenta().bold(),
        "n".magenta().bold(),
        "q".magenta().bold(),
    );

    println!(
        "{}",
        "-------------------------------------------------------------------------------".magenta()
    );
}

pub fn show_menu() {
    println!(
        "\n{}",
        "-------------------------------------------------------------------------------".magenta()
    );
    println!("{}", "** Menu **".blue().bold());
    println!("{}", "Digite uma opção:".yellow());

    println!(
        "\
{}) Verificar se dois vértices informados são adjacentes
{}) Verificar se existe um laço a partir da leitura de um vértice
{}) Verificar se existe e exibir o caminho a partir da leitura de dois vértices
{}) Verificar se existe e exibir o comprimento dado um ciclo
{}) Criar novas arestas
{}) Remover arestas
{}) Tornar o grafo ponderado
{}) Verificar se um dado grafo é subgrafo
{}) Verificar se o grafo é completo
{}) Calcular o custo do caminho entre dois vértices informados
---
{}) Visualizar grafo atual
{}) Salvar grafo atual
{}) Exportar como PNG grafo atual
{}) Encerrar",
        "a".magenta().bold(),
        "b".magenta().bold(),
        "c".magenta().bold(),
        "d".magenta().bold(),
        "e".magenta().bold(),
        "f".magenta().bold(),
        "g".magenta().bold(),
        "h".magenta().bold(),
        "i".magenta().bold(),
        "j".magenta().bold(),
        "v".magenta().bold(),
        "s".magenta().bold(),
        "p".magenta().bold(),
        "q".magenta().bold(),
    );

    println!(
        "{}",
        "-------------------------------------------------------------------------------".magenta()
    );
}

pub fn read_option() -> MenuOpt {
    loop {
        let mut option = String::new();

        io::stdin().read_line(&mut option).unwrap();

        match parse_option(&option.trim()) {
            Some(opt) => {
                // Clear input
                print!("{}", Feedback::option_read(&option));
                break opt;
            }
            None => {
                println!("{}", Feedback::invalid_option());
                continue;
            }
        };
    }
}

fn parse_option(option: &str) -> Option<MenuOpt> {
    match option {
        "a" => Some(A),
        "b" => Some(B),
        "c" => Some(C),
        "d" => Some(D),
        "e" => Some(E),
        "f" => Some(F),
        "g" => Some(G),
        "h" => Some(H),
        "i" => Some(I),
        "j" => Some(J),
        "n" => Some(No),
        "l" => Some(Load),
        "v" => Some(Visualize),
        "s" => Some(Save),
        "p" => Some(Export),
        "q" => Some(Exit),
        _ => None,
    }
}

pub fn run_option(option: MenuOpt, graph: &mut Graph) {
    let result = match option {
        A => verify_if_two_nodes_are_adjacent(graph),
        B => has_buckle_menu(graph),
        C => find_path_menu(graph),
        D => find_and_show_cycle(graph),
        E => add_edge_menu(graph),
        F => remove_edge_menu(graph),
        Save => save_graph(graph),
        Visualize => show_graph(graph),
        Export => export_graph(graph),
        _ => Ok(format!("i")),
    };

    match result {
        Ok(success) => println!("{success}"),
        Err(err) => println!("{err}"),
    }
}

fn verify_if_two_nodes_are_adjacent(graph: &Graph) -> RunOptResult {
    println!("{}", format_available_nodes(graph));

    println!("{}", Feedback::nth_node("Primeiro"));
    let node1 = read_node(graph)?;

    println!("{}", Feedback::nth_node("Segundo"));
    let node2 = read_node(graph)?;

    let result = if node1.is_adjacent(node2) {
        Feedback::adjacent_nodes(node1.code, node2.code)
    } else {
        Feedback::not_adjacent_nodes(node1.code, node2.code)
    };

    Ok(result)
}

fn has_buckle_menu(graph: &mut Graph) -> RunOptResult {
    println!("{}", format_available_nodes(graph));
    let node = read_node(graph)?;
    if node.has_buckle() {
        Ok(Feedback::contains_buckle(node.code))
    } else {
        Ok(Feedback::no_buckle(node.code))
    }
}

fn find_path_menu(graph: &mut Graph) -> RunOptResult {
    let node1 = read_node(graph)?;
    let node2 = read_node(graph)?;

    match graph.get_path(node1, node2) {
        Some(path) => Ok(get_string_path(path)),
        None => return Ok(Feedback::no_path_found(node1.code, node2.code)),
    }

    // "1 -> 2 -> 3"

    // Ok(Feedback::edge_added())
}

fn find_and_show_cycle(graph: &Graph) -> RunOptResult {
    let codes = read_cycle();
    print!("\n");

    match graph.get_cycle(&codes) {
        Some(cycle) => Ok(get_string_path(cycle)),
        None => return Ok(Feedback::no_cycle_found()),
    }
}

fn get_string_path(nodes: Vec<&Node>) -> String {
    nodes
        .iter()
        .map(|x| format!("[{}] {}", x.code.to_string(), x.name))
        .collect::<Vec<_>>()
        .join(" <-> ")
}

fn add_edge_menu(graph: &mut Graph) -> RunOptResult {
    println!("{}\n", format_available_nodes(graph));

    println!("{}", Feedback::nth_node("Primeiro"));
    let code1 = read_node(graph)?.code;

    println!("{}", Feedback::nth_node("Segundo"));
    let code2 = read_node(graph)?.code;

    let edge1 = Edge {
        code: code1,
        weight: 1,
    };
    let edge2 = Edge {
        code: code2,
        weight: 1,
    };

    match graph.add_edge(edge1, edge2) {
        Ok(_) => Ok(Feedback::edge_added(code1, code2)),
        Err(_) => Err(Feedback::edge_already_exists()),
    }
}

fn remove_edge_menu(graph: &mut Graph) -> RunOptResult {
    println!("{}\n", format_available_nodes(graph));

    println!("{}", Feedback::nth_node("Primeiro"));
    let code1 = read_node(graph)?.code;

    println!("{}", Feedback::nth_node("Segundo"));
    let code2 = read_node(graph)?.code;

    let edge1 = Edge {
        code: code1,
        weight: 1,
    };
    let edge2 = Edge {
        code: code2,
        weight: 1,
    };

    match graph.remove_edge(edge1, edge2) {
        Ok(_) => Ok(Feedback::edge_removed(code1, code2)),
        Err(_) => Err(Feedback::edge_dont_exists()),
    }
}

fn save_graph(graph: &Graph) -> RunOptResult {
    let data = serde_json::to_string(graph).unwrap();

    match fs::write(Path::new(FILE_PATH), data) {
        Ok(_) => Ok(Feedback::save_graph_success()),
        Err(_) => Err(Feedback::save_graph_error()),
    }
}

pub fn load_graph() -> Option<Graph> {
    let data = match fs::read_to_string(Path::new(FILE_PATH)) {
        Ok(data) => data,
        Err(_) => return None,
    };

    match serde_json::from_str(&data) {
        Ok(parsed_graph) => parsed_graph,
        Err(_) => None,
    }
}

fn show_graph(graph: &Graph) -> RunOptResult {
    Ok(format_available_nodes(graph))
}

fn export_graph(graph: &Graph) -> RunOptResult {
    Err("oi".to_string())
}

fn read_code() -> usize {
    loop {
        println!("{}", Feedback::read_code());

        let mut code = String::new();

        io::stdin().read_line(&mut code).unwrap();

        match code.trim().parse() {
            Ok(parsed_code) => break parsed_code,
            Err(_) => {
                println!("{}", Feedback::invalid_code());
                continue;
            }
        };
    }
}

fn read_cycle() -> Vec<usize> {
    loop {
        println!("{}", Feedback::read_codes());

        let mut codes = String::new();

        io::stdin().read_line(&mut codes).unwrap();

        let codes_iter = codes.trim().split(",").map(|c| c.trim().parse::<usize>());

        if codes_iter.clone().any(|c| c.is_err()) {
            println!("{}", Feedback::invalid_codes());
            continue;
        }

        let codes = codes_iter.map(|c| c.unwrap()).collect();

        if !Graph::is_cycle(&codes) {
            println!("{}", Feedback::invalid_cycle());
            continue;
        }

        break codes;
    }
}

fn read_node<'a>(graph: &'a Graph) -> Result<&'a Node, String> {
    loop {
        let code = read_code();

        match graph.find_by_code(code) {
            Some(node) => {
                println!("{}", Feedback::code_read(code));
                print!("\n");
                break Ok(node);
            }
            None => {
                println!("{}", Feedback::node_not_found_with_code());
                continue;
            }
        }
    }
}

fn format_available_nodes(graph: &Graph) -> String {
    print!("{}", Feedback::available_nodes());

    let mut string = String::new();
    for node in graph.nodes.iter() {
        string = format!(
            "{string}\n{} - {node}",
            node.code.to_string().magenta().bold()
        );
    }

    string
}
