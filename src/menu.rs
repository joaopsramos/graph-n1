use crate::{graph::Graph, node::Node};
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
        "l".magenta().bold(),
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
{}) Carregar grafo salvo
{}) Visualizar grafo atual
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
        "v".magenta().bold(),
        "s".magenta().bold(),
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
        Save => save_graph(graph),
        Visualize => show_graph(graph),
        Load => load_graph(graph),
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
        .map(|x| x.code.to_string())
        .collect::<Vec<_>>()
        .join(" -> ")
}

fn add_edge_menu(graph: &mut Graph) -> RunOptResult {
    println!("{}\n", format_available_nodes(graph));

    println!("{}", Feedback::nth_node("Primeiro"));
    let code1 = read_node(graph)?.code;

    println!("{}", Feedback::nth_node("Segundo"));
    let code2 = read_node(graph)?.code;

    match graph.add_edge(code1, code2) {
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

    match graph.remove_edge(code1, code2) {
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

    println!("{}\n", Feedback::load_graph_success());

    Ok(format_available_nodes(graph))
}

fn show_graph(graph: &Graph) -> RunOptResult {
    Ok(format_available_nodes(graph))
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

struct Feedback;

impl Feedback {
    fn clear_line() {
        print!("\u{1b}[1F");
    }

    fn option_read(opt: &str) -> String {
        Self::clear_line();
        format!("Opção digitada: {}", opt.magenta())
    }

    fn code_read(code: usize) -> String {
        Self::clear_line();
        format!("Código digitado: {}", code.to_string().magenta())
    }

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
        format!("{}", "Digite um código:".yellow())
    }

    fn read_codes() -> String {
        format!("{}", "Digite os códigos separados por virgula:".yellow())
    }

    fn invalid_codes() -> String {
        format!(
            "{}",
            "Erro ao ler os códigos, eles devem ser inteiros separados por vírgula".red()
        )
    }

    fn invalid_cycle() -> String {
        format!("{}", "Certifique-se de digitar um ciclo válido, o primeiro e o último elemento precisam ser iguais, e não pode haver elementos repetidos entre eles".red())
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

    fn no_buckle(edge: usize) -> String {
        format!("O vértice {edge} não tem um laço")
    }

    fn contains_buckle(edge: usize) -> String {
        format!("O vértice {edge} tem um laço")
    }

    fn no_path_found(edge1: usize, edge2: usize) -> String {
        format!("Não existe caminho entre o vértice {edge1} e o vértice {edge2}")
    }

    fn edge_added(edge1: usize, edge2: usize) -> String {
        format!(
            "{}\n{}",
            "Aresta criada com sucesso".green(),
            Self::format_edge(edge1, edge2)
        )
    }

    fn edge_removed(edge1: usize, edge2: usize) -> String {
        format!(
            "{} {} {}",
            "Aresta".green(),
            Self::format_edge(edge1, edge2),
            "removida com sucesso".green()
        )
    }

    fn adjacent_nodes(edge1: usize, edge2: usize) -> String {
        format!("Os vértices {edge1} e {edge2} são adjacentes.")
    }

    fn no_cycle_found() -> String {
        format!("{}", "Ciclo não encontrado")
    }
    
    fn not_adjacent_nodes(edge1: usize, edge2: usize) -> String {
        format!("Os vértices {edge1} e {edge2} não são adjacentes.")
    }

    fn edge_already_exists() -> String {
        format!("{}", "Aresta já existe".red())
    }

    fn edge_dont_exists() -> String {
        format!("Não existe nenhuma aresta")
    }

    fn format_edge(edge1: usize, edge2: usize) -> String {
        format!("{edge1} <-> {edge2}")
    }
}
