use colored::*;

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
}
