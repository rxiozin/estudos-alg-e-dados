use std::collections::{HashMap, VecDeque};
use crate::modelo::{Produto, Grafo};

pub fn recomendar_produtos<'a>(grafo: &'a Grafo, catalogo: &'a HashMap<u32, Produto>, id_inicial: u32, max_recomendacoes: usize) -> Vec<&'a Produto> {
    let mut visitados: HashMap<u32, bool> = HashMap::new();
    let mut fila: VecDeque<u32> = VecDeque::new();
    let mut recomendacoes: Vec<&Produto> = Vec::new();

    // Inicia pelo produto inicial
    fila.push_back(id_inicial);
    visitados.insert(id_inicial, true);

    while !fila.is_empty() && recomendacoes.len() < max_recomendacoes {
        if let Some(id_atual) = fila.pop_front() {
            if let Some(produto) = catalogo.get(&id_atual) {
                recomendacoes.push(produto);
            }

            // Explora vizinhos no grafo
            if let Some(vizinhos) = grafo.get(&id_atual) {
                for &vizinho in vizinhos {
                    if !visitados.contains_key(&vizinho) {
                        fila.push_back(vizinho);
                        visitados.insert(vizinho, true);
                    }
                }
            }
        }
    }

    recomendacoes
}