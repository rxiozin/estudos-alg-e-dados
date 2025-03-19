mod modelo;
mod catalogo;
mod busca;
mod recomendacao;

use catalogo::criar_catalogo;
use busca::buscar_produto;
use recomendacao::recomendar_produtos;

fn main() {
    // Cria o catálogo e o grafo
    let (catalogo, grafo) = criar_catalogo();

    // Teste de busca (ID 1 = Camiseta)
    match buscar_produto(&catalogo, 1, "senha_secreta") {
        Some(produto) => println!("Produto encontrado: {:?}", produto),
        None => println!("Busca falhou: ID não encontrado ou chave inválida"),
    }

    // Teste de recomendação a partir do ID 1 (Camiseta)
    let recomendacoes = recomendar_produtos(&grafo, &catalogo, 1, 2);
    println!("Recomendações: {:?}", recomendacoes);
}