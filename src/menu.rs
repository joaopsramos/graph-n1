use crate::{
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

fn make_graph_weighted(graph: &mut Graph) -> RunOptResult {
    for node in &mut graph.nodes {
        for edge in &mut node.edges {
            println!("Aresta {}", Feedback::format_edge(node.code, edge.code));

            let weight = read_weight();

            Node::add_weight(edge, weight);

            print!("\n")
        }
    }

    Ok(Feedback::success_graph_weighted())
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
        Ok(data) => {
            println!("\n{}", Feedback::load_graph_success());
            data
        }
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

fn read_node(graph: &Graph) -> Result<&Node, String> {
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

fn read_weight() -> u32 {
    loop {
        println!("{}", Feedback::read_weight());

        let mut weight = String::new();

        io::stdin().read_line(&mut weight).unwrap();

        match weight.trim().parse() {
            Ok(parsed_weight) => break parsed_weight,
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

pub struct Feedback;

impl Feedback {
    pub fn clear_line() {
        print!("\u{1b}[1F");
    }

    pub fn option_read(opt: &str) -> String {
        Self::clear_line();
        format!("Opção digitada: {}", opt.magenta())
    }

    pub fn code_read(code: usize) -> String {
        Self::clear_line();
        format!("Código digitado: {}", code.to_string().magenta())
    }

    pub fn invalid_option() -> String {
        format!(
            "{}",
            "Por favor, digite uma opção válida conforme o menu.".red()
        )
    }

    pub fn node_not_found_with_code() -> String {
        format!(
            "{}",
            "Nenhum vértice foi encontrado com esse código, tente digitar outro...".red()
        )
    }

    pub fn invalid_code() -> String {
        format!("{}", "Por favor, digite um código válido.".red())
    }

    pub fn nth_node(num: &str) -> String {
        let msg = format!("* {num} vértice *");
        format!("{}", msg.blue().bold())
    }

    pub fn read_code() -> String {
        format!("{}", "Digite um código:".yellow())
    }

    pub fn read_codes() -> String {
        format!("{}", "Digite os códigos separados por virgula:".yellow())
    }

    pub fn invalid_codes() -> String {
        format!(
            "{}",
            "Erro ao ler os códigos, eles devem ser inteiros separados por vírgula".red()
        )
    }

    pub fn read_weight() -> String {
        format!("{}", "Digite o peso da aresta:".yellow())
    }

    pub fn invalid_weight() -> String {
        format!(
            "{}",
            "Peso inválido, ele deve ser um inteiro maior que 0".red()
        )
    }

    pub fn invalid_cycle() -> String {
        format!("{}", "Certifique-se de digitar um ciclo válido, o primeiro e o último elemento precisam ser iguais, e não pode haver elementos repetidos entre eles".red())
    }

    pub fn available_nodes() -> String {
        format!("Vértices disponíveis:")
    }

    pub fn load_graph_success() -> String {
        format!("{}", "Grafo carregado com sucesso!".green())
    }

    pub fn load_graph_error() -> String {
        format!(
            "{}",
            "Erro ao carregar grafo, verique se o arquivo não foi alterado após o grafo ter sido salvo".red()
        )
    }

    pub fn read_graph_file_error() -> String {
        format!(
            "{}",
            "Erro ao ler arquivo, verifique se o grafo foi salvo e se o arquivo existe".red()
        )
    }

    pub fn save_graph_success() -> String {
        format!("{}", "Grafo salvo com sucesso!".green())
    }

    pub fn save_graph_error() -> String {
        format!("{}", "Erro ao salvar arquivo :(".red())
    }

    pub fn no_buckle(edge: usize) -> String {
        format!("O vértice {edge} não tem um laço")
    }

    pub fn contains_buckle(edge: usize) -> String {
        format!("O vértice {edge} tem um laço")
    }

    pub fn no_path_found(edge1: usize, edge2: usize) -> String {
        format!("Não existe caminho entre o vértice {edge1} e o vértice {edge2}")
    }

    pub fn edge_added(edge1: usize, edge2: usize) -> String {
        format!(
            "{}\n{}",
            "Aresta criada com sucesso".green(),
            Self::format_edge(edge1, edge2)
        )
    }

    pub fn edge_removed(edge1: usize, edge2: usize) -> String {
        format!(
            "{} {} {}",
            "Aresta".green(),
            Self::format_edge(edge1, edge2),
            "removida com sucesso".green()
        )
    }

    pub fn adjacent_nodes(edge1: usize, edge2: usize) -> String {
        format!("Os vértices {edge1} e {edge2} são adjacentes.")
    }

    pub fn no_cycle_found() -> String {
        format!("{}", "Ciclo não encontrado")
    }
    pub fn not_adjacent_nodes(edge1: usize, edge2: usize) -> String {
        format!("Os vértices {edge1} e {edge2} não são adjacentes.")
    }

    pub fn edge_already_exists() -> String {
        format!("{}", "Aresta já existe".red())
    }

    pub fn edge_dont_exists() -> String {
        format!("Não existe nenhuma aresta")
    }

    pub fn format_edge(edge1: usize, edge2: usize) -> String {
        format!("{edge1} <-> {edge2}")
    }

    pub fn success_graph_weighted() -> String {
        format!("{}", "Grafo ponderado com sucesso!".green())
    }
}
