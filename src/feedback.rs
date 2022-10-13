use crate::graph::Edge;
use colored::Colorize;

pub struct Feedback;

impl Feedback {
    pub fn clear_line() {
        print!("\u{1b}[1F");
    }

    pub fn value_read(value: &str, text: &str) -> String {
        Self::clear_line();
        format!("{text}: {}", value.magenta())
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
        format!("A aresta informada não existe")
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

    pub fn graph_is_complete() -> String {
        format!("O grafo {} completo", "é".green())
    }

    pub fn graph_is_not_complete() -> String {
        format!("O grafo {} completo", "não é".red())
    }

    pub fn graph_is_not_weighted() -> String {
        format!("{}", "O grafo precisa ser ponderado".red())
    }

    pub fn path_size(size: u32) -> String {
        format!("O tamanho do caminho é: {}", size.to_string().green())
    }

    pub fn is_subgraph() -> String {
        format!("O grafo informado {} subgrafo do atual", "é".green())
    }

    pub fn is_not_subgraph() -> String {
        format!("O grafo informado {} subgrafo do atual", "não é".red())
    }

    pub fn graph_exported(path: &str) -> String {
        let text = format!("{}", "O grafo foi exportado com sucesso!".green());
        format!("{text} Arquivo: {path}")
    }

    pub fn graph_not_exported() -> String {
        let executable = format!("{}", "graphviz".green());
        let link = format!("{}", "https://graphviz.org/".cyan());

        format!("{}\nVerifique se você possui o programa {executable} ({link}) instalado e se é possivel criar arquivos na pasta atual", "Erro ao exportar grafo".red())
    }
}
