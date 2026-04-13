
use megastore_busca::*; 

#[test]
fn test_buscar_produto_existente() {
    let mut store = CatalogoMegaStore::new();
    store.adicionar_produto("mouse", 80.0, "acessorio");
    
    let resultado = store.buscar("mouse");
    assert!(resultado.is_some());
    assert_eq!(resultado.unwrap().preco, 80.0);
}

#[test]
fn test_recomendacao_por_grafo() {
    let mut store = CatalogoMegaStore::new();
    store.adicionar_produto("teclado", 150.0, "perifericos");
    store.adicionar_produto("mouse", 80.0, "perifericos");
    
 
    store.conectar_produtos("teclado", "mouse");
    
    let recomendacoes = store.obter_recomendacoes("teclado");
    assert_eq!(recomendacoes.len(), 1);
    assert_eq!(recomendacoes[0].nome, "mouse");
}
