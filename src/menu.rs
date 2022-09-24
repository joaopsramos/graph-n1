use crate::{graph::Graph, node::Node};
use colored::Colorize;
use std::{fs, io, path::Path};
use MenuOpt::*;

type RunOptResult = Result<String, String>;

const FILE_PATH: &str = "./graph.json";

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
    Load,
    Save,
    Exit,
}

pub fn show_menu() {
    println!("\n----------");
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
{}) Carregar grafo salvo
{}) Salvar grafo atual
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
        "l".magenta().bold(),
        "s".magenta().bold(),
        "q".magenta().bold()
    );
}

pub fn read_option() -> MenuOpt {
    loop {
        let mut option = String::new();

        io::stdin().read_line(&mut option).unwrap();

        match parse_option(&option.trim()) {
            Some(opt) => break opt,
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
        "l" => Some(Load),
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
        E => add_edge_menu(graph),
        F => remove_edge_menu(graph),
        Save => save_graph(graph),
        Load => load_graph(graph),
        _ => Ok(format!("i")),
    };

    match result {
        Ok(success) => println!("{success}"),
        Err(err) => println!("{err}"),
    }
}

fn verify_if_two_nodes_are_adjacent(graph: &Graph) -> RunOptResult {
    Feedback::nth_node("Primeiro");
    let node1 = read_node(graph)?;

    Feedback::nth_node("Segundo");
    let node2 = read_node(graph)?;

    let result = if node1.is_adjacent(node2) {
        Feedback::adjacent_nodes(node1, node2)
    } else {
        Feedback::not_adjacent_nodes(node1, node2)
    };

    Ok(result)
}

fn has_buckle_menu(graph: &mut Graph) -> RunOptResult {
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

fn get_string_path(nodes: Vec<&Node>) -> String {
    nodes
        .iter()
        .map(|x| x.code.to_string())
        .collect::<Vec<_>>()
        .join(" -> ")
}

fn add_edge_menu(graph: &mut Graph) -> RunOptResult {
    let graph_clone = &graph.clone();

    let node1 = read_node_mut(graph)?;
    let node2 = read_node(graph_clone)?;

    node1.add_edge(node2.code);

    Ok(Feedback::edge_added())
}

fn remove_edge_menu(graph: &mut Graph) -> RunOptResult {
    let graph_clone = &graph.clone();

    let node1 = read_node_mut(graph)?;
    let node2 = read_node(graph_clone)?;

    node1.remove_edge(&node2);

    Ok(Feedback::edge_removed())
}

fn save_graph(graph: &Graph) -> RunOptResult {
    let data = serde_json::to_string(graph).unwrap();

    match fs::write(Path::new(FILE_PATH), data) {
        Ok(_) => Ok(Feedback::save_graph_success()),
        Err(_) => Err(Feedback::save_graph_error()),
    }
}

fn load_graph(graph: &mut Graph) -> RunOptResult {
    let data = match fs::read_to_string(Path::new(FILE_PATH)) {
        Ok(data) => data,
        Err(_) => {
            return Err(Feedback::read_graph_file_error());
        }
    };

    *graph = match serde_json::from_str(&data) {
        Ok(parsed_graph) => parsed_graph,
        Err(_) => {
            return Err(Feedback::load_graph_error());
        }
    };

    Ok(Feedback::load_graph_success())
}

fn read_code(graph: &Graph) -> usize {
    loop {
        show_available_nodes(graph);
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

fn read_node<'a>(graph: &'a Graph) -> Result<&'a Node, String> {
    loop {
        let code = read_code(graph);

        match graph.find_by_code(code) {
            Some(node) => break Ok(node),
            None => {
                println!("{}", Feedback::node_not_found_with_code());
                continue;
            }
        }
    }
}

fn read_node_mut(graph: &mut Graph) -> Result<&mut Node, String> {
    let graph_clone = graph.clone();

    let code = loop {
        let code = read_code(&graph_clone);

        match graph.find_by_code(code) {
            Some(_) => break code,
            None => {
                println!("{}", Feedback::node_not_found_with_code());
                continue;
            }
        }
    };

    Ok(graph.find_by_code_mut(code).unwrap())
}

fn show_available_nodes(graph: &Graph) {
    println!("{}", Feedback::available_nodes());

    for node in graph.nodes.iter() {
        println!("{} - {node}", node.code.to_string().magenta().bold());
    }
}

struct Feedback;

impl Feedback {
    fn invalid_option() -> String {
        format!(
            "{}",
            "Por favor, digite uma opção válida conforme o menu.".red()
        )
    }

    fn node_not_found_with_code() -> String {
        format!(
            "{}",
            "Nenhum vértice foi encontrado com esse código, tente digitar outro...".red()
        )
    }

    fn invalid_code() -> String {
        format!("{}", "Por favor, digite um código válido.".red())
    }

    fn nth_node(num: &str) -> String {
        let msg = format!("* {num} vértice *");
        format!("{}", msg.blue().bold())
    }

    fn read_code() -> String {
        format!("\n{}", "Digite um código:".yellow())
    }

    fn available_nodes() -> String {
        format!("Vértices disponíveis:")
    }

    fn load_graph_success() -> String {
        format!("{}", "Grafo carregado com sucesso!".green())
    }

    fn load_graph_error() -> String {
        format!(
            "{}",
            "Erro ao carregar grafo, verique se o arquivo não foi alterado após o grafo ter sido salvo".red()
        )
    }

    fn read_graph_file_error() -> String {
        format!(
            "{}",
            "Erro ao ler arquivo, verifique se o grafo foi salvo e se o arquivo existe".red()
        )
    }

    fn save_graph_success() -> String {
        format!("{}", "Grafo salvo com sucesso!".green())
    }

    fn save_graph_error() -> String {
        format!("{}", "Erro ao salvar arquivo :(".red())
    }

    fn no_buckle(node: usize) -> String {
        format!("O vértice {node} não tem um laço")
    }

    fn contains_buckle(node: usize) -> String {
        format!("O vértice {node} tem um laço")
    }

    fn no_path_found(node1: usize, node2: usize) -> String {
        format!("Não existe caminho entre o vértice {node1} e o vértice {node2}")
    }

    fn edge_added() -> String {
        format!("Aresta criada com sucesso")
    }

    fn edge_removed() -> String {
        format!("Aresta removida com sucesso")
    }

    fn adjacent_nodes(node1: &Node, node2: &Node) -> String {
        format!("Os vértices {node1} e {node2} são adjacentes.")
    }

    fn not_adjacent_nodes(node1: &Node, node2: &Node) -> String {
        format!("Os vértices {node1} e {node2} não são adjacentes.")
    }
}
