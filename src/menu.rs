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
                println!(
                    "{}",
                    "Por favor, digite uma opção válida conforme o menu.".red()
                );
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
    println!("{}", "* Primeiro vértice *".blue().bold());
    let node1 = read_node(graph)?;

    println!("\n{}", "* Segundo vértice *".blue().bold());
    let node2 = read_node(graph)?;

    let result = if node1.is_adjacent(node2) {
        format!("Os vértices {node1} e {node2} são adjacentes.")
    } else {
        format!("Os vértices {node1} e {node2} não são adjacentes.")
    };

    Ok(result)
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

fn read_node(graph: &Graph) -> Result<&Node, String> {
    loop {
        show_available_nodes(graph);
        println!("\n{}", "Digite um código:".yellow());

        let mut code = String::new();

        io::stdin().read_line(&mut code).unwrap();

        let code: u32 = match code.trim().parse() {
            Ok(parsed_code) => parsed_code,
            Err(_) => {
                println!("{}", "Por favor, digite um código válido.".red());
                continue;
            }
        };

        match graph.find_by_code(code) {
            Some(node) => break Ok(node),
            None => {
                println!(
                    "{}",
                    "Nenhum vértice foi encontrado com esse código, tente digitar outro...".red()
                );
                continue;
            }
        }
    }
}

fn show_available_nodes(graph: &Graph) {
    println!("Vértices disponíveis:");

    for node in graph.nodes.iter() {
        println!("{} - {node}", node.code.to_string().magenta().bold());
    }
}
