use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Produto {
    pub id: u32,
    pub nome: String,
    pub categoria: String,
    pub marca: String,
}

pub struct SistemaBusca {
    index_nome: HashMap<String, Vec<Produto>>,
    index_categoria: HashMap<String, Vec<Produto>>,
}

impl SistemaBusca {
    pub fn new() -> Self {
        Self {
            index_nome: HashMap::new(),
            index_categoria: HashMap::new(),
        }
    }

    pub fn adicionar_produto(&mut self, produto: Produto) {
        self.index_nome
            .entry(produto.nome.clone().to_lowercase())
            .or_insert(Vec::new())
            .push(produto.clone());

        self.index_categoria
            .entry(produto.categoria.clone().to_lowercase())
            .or_insert(Vec::new())
            .push(produto);
    }

    pub fn buscar_por_nome(&self, nome: &str) -> Option<&Vec<Produto>> {
        self.index_nome.get(&nome.to_lowercase())
    }
}