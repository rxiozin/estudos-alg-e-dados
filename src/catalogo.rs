use std::collections::HashMap;
use crate::modelo::{Produto, Grafo};

pub fn criar_catalogo() -> (HashMap<u32, Produto>, Grafo) {
    let mut catalogo: HashMap<u32, Produto> = HashMap::new();
    let mut grafo: Grafo = HashMap::new();

    adicionar_produto(&mut catalogo, 1, "Camiseta".to_string(), 29.99, "Roupas".to_string()).unwrap();
    adicionar_produto(&mut catalogo, 2, "Calça".to_string(), 59.99, "Roupas".to_string()).unwrap();
    adicionar_produto(&mut catalogo, 3, "Tênis".to_string(), 89.99, "Calçados".to_string()).unwrap();

    grafo.insert(1, vec![2]);
    grafo.insert(2, vec![1, 3]);
    grafo.insert(3, vec![2]);

    (catalogo, grafo)
}

fn adicionar_produto(catalogo: &mut HashMap<u32, Produto>, id: u32, nome: String, preco: f32, categoria: String) -> Result<(), String> {
    if id == 0 || preco < 0.0 || nome.is_empty() {
        return Err("Dados inválidos: ID deve ser > 0, preço >= 0 e nome não vazio".to_string());
    }
    let produto = Produto { id, nome, preco, categoria };
    catalogo.insert(id, produto);
    Ok(())
}