use crate::{
    feedback::Feedback,
    graph::{Edge, Graph},
    graph_builder,
    node::Node,
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
{}) Visualizar grafo
{}) Salvar grafo
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
        "q".magenta().bold(),
    );

    println!(
        "{}",
        "-------------------------------------------------------------------------------".magenta()
    );

    println!("{}", "Digite uma opção:".yellow());
}

pub fn read_option() -> MenuOpt {
    loop {
        let mut option = String::new();

        io::stdin().read_line(&mut option).unwrap();

        match parse_option(&option.trim()) {
            Some(opt) => {
                // Clear input
                print!("{}", Feedback::value_read(&option, "Opção digitada"));
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
        G => make_graph_weighted(graph),
        H => verify_if_graph_contains_subgraph(graph),
        I => is_graph_complete(graph),
        J => calc_path_between_nodes(graph),
        Save => save_graph(graph),
        Visualize => show_graph(graph),
        _ => Ok(format!("i")),
    };

    match result {
        Ok(success) => println!("{success}"),
        Err(err) => println!("{err}"),
    }
}

fn verify_if_two_nodes_are_adjacent(graph: &Graph) -> RunOptResult {
    println!("{}\n", format_available_nodes(graph));

    println!("{}", Feedback::nth_node("Primeiro"));
    let node1 = read_node(graph)?;

    println!("\n{}", Feedback::nth_node("Segundo"));
    let node2 = read_node(graph)?;

    let result = if graph.is_adjacent(node1, node2) {
        Feedback::adjacent_nodes(node1.code, node2.code)
    } else {
        Feedback::not_adjacent_nodes(node1.code, node2.code)
    };

    Ok(result)
}

fn has_buckle_menu(graph: &mut Graph) -> RunOptResult {
    println!("{}\n", format_available_nodes(graph));

    let node = read_node(graph)?;

    if graph.has_buckle(&node) {
        Ok(Feedback::contains_buckle(node.code))
    } else {
        Ok(Feedback::no_buckle(node.code))
    }
}

fn find_path_menu(graph: &mut Graph) -> RunOptResult {
    println!("{}", Feedback::nth_node("Primeiro"));
    let node1 = read_node(graph)?;

    println!("\n{}", Feedback::nth_node("Segundo"));
    let node2 = read_node(graph)?;

    print!("\n");

    match graph.get_path(node1, node2) {
        Some(path) => {
            println!("{}", Feedback::path_found());
            Ok(get_string_path(path))
        }
        None => return Ok(Feedback::no_path_found(node1.code, node2.code)),
    }
}

fn find_and_show_cycle(graph: &Graph) -> RunOptResult {
    let codes = read_cycle();
    print!("\n");

    match graph.get_cycle(&codes) {
        Some(cycle) => {
            println!("{}", Feedback::cycle_found());
            Ok(get_string_path(cycle))
        }
        None => Ok(Feedback::no_cycle_found()),
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
    let default_weight = 1;

    println!("{}\n", format_available_nodes(graph));

    println!("{}", Feedback::nth_node("Primeiro"));
    let from = read_node(graph)?.code;

    println!("\n{}", Feedback::nth_node("Segundo"));
    let to = read_node(graph)?.code;

    let weight = if graph.is_weighted {
        print!("\n");
        read_weight()
    } else {
        default_weight
    };

    let edge = Edge { from, to, weight };

    print!("\n");
    match graph.add_edge(edge.clone()) {
        Ok(_) => Ok(Feedback::edge_added(edge)),
        Err(_) => Err(Feedback::edge_already_exists()),
    }
}

fn remove_edge_menu(graph: &mut Graph) -> RunOptResult {
    println!("{}\n", format_available_nodes(graph));

    println!("{}", Feedback::nth_node("Primeiro"));
    let from = read_node(graph)?.code;

    println!("\n{}", Feedback::nth_node("Segundo"));
    let to = read_node(graph)?.code;

    print!("\n");
    match graph.remove_edge(from, to) {
        Ok(_) => Ok(Feedback::edge_removed(from, to)),
        Err(_) => Err(Feedback::edge_dont_exists()),
    }
}

fn make_graph_weighted(graph: &mut Graph) -> RunOptResult {
    if graph.is_weighted {
        return Err(Feedback::graph_already_weighted());
    }

    graph.make_weighted();

    for edge in &mut graph.edges {
        println!("Aresta {}", Feedback::format_edge(edge.from, edge.to));

        let weight = read_weight();

        Graph::add_weight(edge, weight);

        print!("\n")
    }

    Ok(Feedback::success_graph_weighted())
}

fn verify_if_graph_contains_subgraph(graph: &Graph) -> RunOptResult {
    let subgraph = graph_builder::read_subgraph(graph.is_weighted);

    if graph.is_subgraph(&subgraph) {
        Ok(Feedback::is_subgraph())
    } else {
        Ok(Feedback::is_not_subgraph())
    }
}

fn is_graph_complete(graph: &Graph) -> RunOptResult {
    if graph.is_complete() {
        Ok(Feedback::graph_is_complete())
    } else {
        Ok(Feedback::graph_is_not_complete())
    }
}

fn calc_path_between_nodes(graph: &Graph) -> RunOptResult {
    if !graph.is_weighted {
        return Err(Feedback::graph_is_not_weighted());
    }

    println!("{}", Feedback::nth_node("Primeiro"));
    let node1 = read_node(graph)?;

    println!("\n{}", Feedback::nth_node("Segundo"));
    let node2 = read_node(graph)?;

    print!("\n");

    match graph.calculate_path(node1, node2) {
        Some(path_size) => Ok(Feedback::path_size(path_size)),
        None => Err(Feedback::no_path_found(node1.code, node2.code)),
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
        Ok(parsed_graph) => {
            println!("\n{}", Feedback::load_graph_success());
            parsed_graph
        }
        Err(_) => None,
    }
}

fn show_graph(graph: &Graph) -> RunOptResult {
    println!("{}", "** Grafo **".blue().bold());
    Ok(graph.to_string())
}

fn read_code() -> usize {
    loop {
        println!("{}", Feedback::read_code());

        let mut code = String::new();

        io::stdin().read_line(&mut code).unwrap();
        println!("{}", Feedback::value_read(&code, "Código digitado"));

        match code.trim().parse::<usize>() {
            Ok(parsed_code) => {
                break parsed_code;
            }
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
            println!("{}\n", Feedback::invalid_codes());
            continue;
        }

        let codes = codes_iter.map(|c| c.unwrap()).collect();

        if !Graph::is_cycle(&codes) {
            println!("{}\n", Feedback::invalid_cycle());
            continue;
        }

        println!("{}", Feedback::cycle_read(&codes));

        break codes;
    }
}

fn read_node(graph: &Graph) -> Result<&Node, String> {
    loop {
        let code = read_code();

        match graph.find_by_code(code) {
            Some(node) => break Ok(node),
            None => {
                println!("{}\n", Feedback::node_not_found_with_code());
                continue;
            }
        }
    }
}

fn read_weight() -> u32 {
    loop {
        println!("{}", Feedback::read_weight());

        let mut weight = String::new();

        io::stdin().read_line(&mut weight).unwrap();
        println!("{}", Feedback::value_read(&weight, "Peso digitado"));

        match weight.trim().parse() {
            Ok(parsed_weight) => {
                break parsed_weight;
            }
            Err(_) => {
                println!("{}", Feedback::invalid_weight());
                continue;
            }
        };
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
