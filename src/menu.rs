use crate::{graph::Graph, node::Node};
use colored::Colorize;
use std::{fs, io, path::Path};
use MenuOpt::*;

type RunOptResult = Result<String, String>;

const FILE_PATH: &str = "./graph.json";

struct Print;

impl Print {
    fn invalid_option() {
        println!(
            "{}",
            "Por favor, digite uma opção válida conforme o menu.".red()
        );
    }

    fn node_not_found_with_code() {
        println!(
            "{}",
            "Nenhum vértice foi encontrado com esse código, tente digitar outro...".red()
        );
    }

    fn invalid_code() {
        println!("{}", "Por favor, digite um código válido.".red());
    }

    fn nth_node(num: &str) {
        let msg = format!("* {num} vértice *");
        println!("{}", msg.blue().bold());
    }
}

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
                Print::invalid_option();
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
    Print::nth_node("Primeiro");
    let node1 = read_node(graph)?;

    Print::nth_node("Segundo");
    let node2 = read_node(graph)?;

    let result = if node1.is_adjacent(node2) {
        format!("Os vértices {node1} e {node2} são adjacentes.")
    } else {
        format!("Os vértices {node1} e {node2} não são adjacentes.")
    };

    Ok(result)
}

fn remove_edge_menu(graph: &mut Graph) -> RunOptResult {
    let graph_clone = &graph.clone();

    let node1 = read_node_mut(graph)?;
    let node2 = read_node(graph_clone)?;

    node1.remove_edge(&node2);

    Ok(format!("Removido com sucesso!"))
}

fn save_graph(graph: &Graph) -> RunOptResult {
    let data = serde_json::to_string(graph).unwrap();

    match fs::write(Path::new(FILE_PATH), data) {
        Ok(_) => Ok(format!("{}", "Grafo salvo com sucesso!".green())),
        Err(_) => Err(format!("{}", "Erro ao salvar arquivo :(".red())),
    }
}

fn load_graph(graph: &mut Graph) -> RunOptResult {
    let data = match fs::read_to_string(Path::new(FILE_PATH)) {
        Ok(data) => data,
        Err(_) => {
            return Err(format!(
                "{}",
                "Erro ao ler arquivo, verifique se o grafo foi salvo e se o arquivo existe".red()
            ));
        }
    };

    *graph = match serde_json::from_str(&data) {
        Ok(parsed_graph) => parsed_graph,
        Err(_) => {
            return Err(format!("{}", "Erro ao carregar grafo, verique se o arquivo não foi alterado após o grafo ter sido salvo".red()));
        }
    };

    Ok(format!("{}", "Grafo carregado com sucesso!".green()))
}

fn read_code(graph: &Graph) -> usize {
    loop {
        show_available_nodes(graph);
        println!("\n{}", "Digite um código:".yellow());

        let mut code = String::new();

        io::stdin().read_line(&mut code).unwrap();

        match code.trim().parse() {
            Ok(parsed_code) => break parsed_code,
            Err(_) => {
                Print::invalid_code();
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
                Print::node_not_found_with_code();
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
                Print::node_not_found_with_code();
                continue;
            }
        }
    };

    Ok(graph.find_by_code_mut(code).unwrap())
}
fn show_available_nodes(graph: &Graph) {
    println!("Vértices disponíveis:");

    for node in graph.nodes.iter() {
        println!("{} - {node}", node.code.to_string().magenta().bold());
    }
}
