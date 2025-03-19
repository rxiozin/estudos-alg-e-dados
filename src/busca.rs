use std::collections::HashMap;
use crate::modelo::Produto;

pub fn buscar_produto<'a>(catalogo: &'a HashMap<u32, Produto>, id: u32, chave_acesso: &str) -> Option<&'a Produto> {
    println!("Debug: Chave recebida: '{}'", chave_acesso);
    if chave_acesso != "senha_secreta" {
        println!("Debug: Chave inválida");
        return None;
    }
    println!("Debug: Buscando ID {}", id);
    let resultado = catalogo.get(&id);
    println!("Debug: Resultado da busca: {:?}", resultado);
    resultado
}