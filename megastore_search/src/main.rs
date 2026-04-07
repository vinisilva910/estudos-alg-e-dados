use megastore_search::{Produto, SistemaBusca};

fn main() {
    let mut sistema = SistemaBusca::new();

    sistema.adicionar_produto(Produto {
        id: 1,
        nome: "Notebook".to_string(),
        categoria: "Eletronicos".to_string(),
        marca: "Dell".to_string(),
    });

    sistema.adicionar_produto(Produto {
        id: 2,
        nome: "Mouse".to_string(),
        categoria: "Eletronicos".to_string(),
        marca: "Logitech".to_string(),
    });

    println!("🔎 Buscando por: Notebook");

    if let Some(resultados) = sistema.buscar_por_nome("Notebook") {
        for produto in resultados {
            println!("Produto encontrado: {:?}", produto);
        }
    } else {
        println!("Nenhum produto encontrado");
    }
}