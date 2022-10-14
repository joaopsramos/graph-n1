use crate::{feedback::Feedback, graph::*, node::*, GRAPH_SIZE};
use colored::*;
use std::{io, str::FromStr};

pub fn init_graph() -> Graph {
    let mut nodes = Vec::new();

    let size = format!("{}", GRAPH_SIZE.to_string().green());
    let text = format!("\nMonte seu grafo com {size} vértices").cyan();
    println!("{}", text.bold().italic());

    for i in 1..=GRAPH_SIZE {
        let mut name = String::new();
        let mut local_type = String::new();

        print!("\n------------------\n");
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
            });

            break;
        }
    }

    Graph {
        is_weighted: false,
        size: GRAPH_SIZE,
        nodes,
        edges: Vec::new(),
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
        println!("{}", Feedback::value_read(&value, "Valor digitado"));

        match value.trim().parse() {
            Ok(parsed_value) => return parsed_value,
            Err(_) => {
                println!("{}", error_msg.red());
                continue;
            }
        };
    }
}

pub fn read_subgraph(is_weighted: bool) -> Graph {
    println!("{}", "Monte seu subgrafo".cyan().bold().italic());

    println!("\n{}", "** Vértices **".blue().bold());
    let nodes = read_nodes();

    println!("{}", "Vértices lidos:".green());
    print_nodes(&nodes);

    let mut graph = Graph {
        is_weighted,
        size: nodes.len(),
        nodes,
        edges: Vec::new(),
    };

    println!("\n-----------------");
    println!("{}", "** Arestas **".blue().bold());
    read_edges(&mut graph, is_weighted);

    println!("{}", "Subgrafo criado:".green());
    println!("{graph}\n");

    graph
}

fn read_nodes() -> Vec<Node> {
    let mut nodes = Vec::new();

    loop {
        let mut name = String::new();
        let mut local_type = String::new();

        println!("-----------------");

        let code = loop {
            let mut code = String::new();

            let quit_opt = format!("{}", "q".purple());
            let text = format!(
                "Digite o código do vértice ou {quit_opt} para terminar a criação dos vértices:"
            );
            println!("{}", text.yellow());

            io::stdin().read_line(&mut code).unwrap();
            println!("{}", Feedback::value_read(&code, "Código digitado"));

            if code.trim() == "q" {
                if nodes.is_empty() {
                    println!("{}\n", "O grafo deve ter pelo menos um vértice!".red());
                    continue;
                }

                return nodes;
            }

            match code.trim().parse() {
                Ok(parsed_code) if parsed_code > 0 => {
                    if node_exists(&nodes, parsed_code) {
                        println!("{}\n", "Erro, um vértice com esse código já existe!".red());
                        continue;
                    }

                    break parsed_code;
                }
                _ => {
                    println!(
                        "{}\n",
                        "Erro, o código precisa ser um número inteiro maior que 0".red()
                    );
                    continue;
                }
            }
        };

        let name: String = read_value("Nome do local:", &mut name, None);
        let local_type: String = read_value("Tipo do local:", &mut local_type, None);

        nodes.push(Node {
            code,
            name,
            local_type,
        });
    }
}

fn read_edges(graph: &mut Graph, is_weighted: bool) {
    let default_weight = 1;

    let quit_opt = format!("{}", "q".purple());
    let text = format!("Digite {quit_opt} a qualquer momento para terminar a criação das arestas");
    println!("\n{}\n", text);

    loop {
        let mut from = String::new();
        let mut to = String::new();

        print!("-----------------\n");

        println!("{}", "Primeiro vértice da aresta".blue());
        let from = match read_code(&graph.nodes, &mut from) {
            Some(from) => from,
            None => break,
        };

        println!("{}", "Segundo vértice da aresta".blue());
        let to = match read_code(&graph.nodes, &mut to) {
            Some(to) => to,
            None => break,
        };

        let weight = if is_weighted {
            read_weight()
        } else {
            Some(default_weight)
        };

        if weight.is_none() {
            break;
        }

        graph
            .add_edge(Edge {
                from,
                to,
                weight: weight.unwrap(),
            })
            .unwrap_or_else(|_err| println!("{}", "Aresta já existe!".red()));
    }
}

fn read_code(nodes: &Vec<Node>, code: &mut String) -> Option<usize> {
    loop {
        *code = String::new();

        println!("{}", "Digite o código:".yellow());

        io::stdin().read_line(code).unwrap();
        println!("{}", Feedback::value_read(code, "Código digitado"));

        if code.trim() == "q" {
            return None;
        }

        match code.trim().parse() {
            Ok(parsed_code) if parsed_code > 0 => {
                if !node_exists(nodes, parsed_code) {
                    println!(
                        "{}\n",
                        "Erro, nenhum vértice encontrado com o código informado".red()
                    );

                    continue;
                }

                return Some(parsed_code);
            }

            _ => {
                println!(
                    "{}\n",
                    "Erro, o código precisa ser um número inteiro maior que 0".red()
                );

                continue;
            }
        }
    }
}

fn read_weight() -> Option<u32> {
    loop {
        let mut weight = String::new();

        println!("{}", "Digite o peso da aresta:".yellow());

        io::stdin().read_line(&mut weight).unwrap();
        println!("{}", Feedback::value_read(&weight, "Peso digitado"));

        if weight.trim() == "q" {
            return None;
        }

        match weight.trim().parse() {
            Ok(parsed_weight) if parsed_weight > 0 => {
                break Some(parsed_weight);
            }

            _ => {
                println!("{}", "Erro, o peso precisa ser maior que 0".red());
                continue;
            }
        }
    }
}

fn node_exists(nodes: &Vec<Node>, code: usize) -> bool {
    nodes.iter().find(|n| n.code == code).is_some()
}

fn print_nodes(nodes: &Vec<Node>) {
    for node in nodes.iter() {
        println!("{node}");
    }
}
