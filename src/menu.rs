use crate::{
    graph::{Edge, Graph},
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

    println!("{}", Feedback::nth_node("Segundo"));
    let to = read_node(graph)?.code;

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
    Ok(format_available_nodes(graph))
}

fn read_code() -> usize {
    loop {
        println!("{}", Feedback::read_code());

        let mut code = String::new();

        io::stdin().read_line(&mut code).unwrap();

        match code.trim().parse() {
            Ok(parsed_code) => {
                println!("{}", Feedback::code_read(parsed_code));
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

        match weight.trim().parse() {
            Ok(parsed_weight) => {
                println!("{}", Feedback::weight_read(parsed_weight));
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

    pub fn weight_read(weight: u32) -> String {
        Self::clear_line();
        format!("Peso digitado: {}", weight.to_string().magenta())
    }

    pub fn cycle_read(cycle: &Vec<usize>) -> String {
        Self::clear_line();

        let cycle = cycle
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<_>>()
            .join(", ");

        format!("Ciclo digitado: {}", cycle.magenta())
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
        format!("{}", "Digite o código:".yellow())
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

    pub fn no_buckle(code: usize) -> String {
        format!(
            "\nO vértice {} {} um laço",
            code.to_string().green(),
            "não possui".red()
        )
    }

    pub fn contains_buckle(code: usize) -> String {
        format!(
            "\nO vértice {} {} um laço",
            code.to_string().green(),
            "possui".cyan()
        )
    }

    pub fn path_found() -> String {
        format!("{}", "Caminho encontrado!".green())
    }

    pub fn no_path_found(code1: usize, code2: usize) -> String {
        format!(
            "{} existe caminho entre o vértice {} e o vértice {}",
            "Não".red(),
            code1.to_string().green(),
            code2.to_string().green()
        )
    }

    pub fn edge_added(edge: Edge) -> String {
        format!(
            "{}\n{}",
            "Aresta criada com sucesso".green(),
            Self::format_edge(edge.from, edge.to)
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

    pub fn adjacent_nodes(code1: usize, code2: usize) -> String {
        format!(
            "\nOs vértices {} e {} {} adjacentes.",
            code1.to_string().green(),
            code2.to_string().green(),
            "são".cyan()
        )
    }

    pub fn not_adjacent_nodes(code1: usize, code2: usize) -> String {
        format!(
            "\nOs vértices {} e {} {} adjacentes.",
            code1.to_string().green(),
            code2.to_string().green(),
            "não são".red()
        )
    }

    pub fn cycle_found() -> String {
        format!("{}", "Ciclo encontrado!".green())
    }

    pub fn no_cycle_found() -> String {
        format!("Ciclo {} encontrado", "não".red())
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
    pub fn graph_already_weighted() -> String {
        format!("{}", "O grafo já é ponderado".red())
    }
}
