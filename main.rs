use std::collections::HashMap;
use std::io::{self, Write};

#[derive(Debug, Clone)]
struct Produto {
    nome: String,
    preco: f64,
    categoria: String,
}


struct CatalogoMegaStore {
    produtos: HashMap<String, Produto>,
    recomendacoes: HashMap<String, Vec<String>>,
}

impl CatalogoMegaStore {
    fn new() -> Self {
        Self {
            produtos: HashMap::new(),
            recomendacoes: HashMap::new(),
        }
    }

    fn adicionar_produto(&mut self, nome: &str, preco: f64, categoria: &str) {
        let p = Produto {
            nome: nome.to_lowercase(),
            categoria: categoria.to_lowercase(),
            preco,
        };
        self.produtos.insert(p.nome.clone(), p);
    }

    fn conectar_produtos(&mut self, p1: &str, p2: &str) {
        self.recomendacoes.entry(p1.to_lowercase()).or_insert(Vec::new()).push(p2.to_lowercase());
        self.recomendacoes.entry(p2.to_lowercase()).or_insert(Vec::new()).push(p1.to_lowercase());
    }

    fn buscar(&self, nome: &str) -> Option<&Produto> {
        self.produtos.get(&nome.to_lowercase())
    }

    fn obter_recomendacoes(&self, nome: &str) -> Vec<&Produto> {
        if let Some(relacionados) = self.recomendacoes.get(&nome.to_lowercase()) {
            relacionados.iter().filter_map(|n| self.produtos.get(n)).collect()
        } else {
            vec![]
        }
    }
}

fn main() {
    let mut store = CatalogoMegaStore::new();
    store.adicionar_produto("celular", 1000.0, "eletronico");
    store.adicionar_produto("carregador", 50.0, "acessorio");
    store.conectar_produtos("celular", "carregador");

    println!("--- Sistema MegaStore ---");
    print!("Digite o nome do produto: ");
    io::stdout().flush().unwrap();

    let mut busca = String::new();
    io::stdin().read_line(&mut busca).expect("Erro ao ler");
    let busca = busca.trim();

    match store.buscar(busca) {
        Some(p) => {
            println!("\nProduto: {} | Preço: R${:.2}", p.nome, p.preco);
            let recs = store.obter_recomendacoes(busca);
            if !recs.is_empty() {
                println!("Recomendações:");
                for r in recs { println!(" - {}", r.nome); }
            }
        }
        None => println!("Não encontrado."),
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_busca() {
        let mut store = CatalogoMegaStore::new();
        store.adicionar_produto("mouse", 50.0, "acessorio");
        assert!(store.buscar("mouse").is_some());
    }

    #[test]
    fn test_grafo() {
        let mut store = CatalogoMegaStore::new();
        store.adicionar_produto("a", 1.0, "cat");
        store.adicionar_produto("b", 1.0, "cat");
        store.conectar_produtos("a", "b");
        assert_eq!(store.obter_recomendacoes("a").len(), 1);
    }
}