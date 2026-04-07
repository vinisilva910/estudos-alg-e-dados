use megastore_search::{Produto, SistemaBusca};

#[test]
fn test_busca_por_nome() {
    let mut sistema = SistemaBusca::new();

    sistema.adicionar_produto(Produto {
        id: 1,
        nome: "Teclado".to_string(),
        categoria: "Eletronicos".to_string(),
        marca: "Razer".to_string(),
    });

    let resultado = sistema.buscar_por_nome("Teclado");

    assert!(resultado.is_some());
}