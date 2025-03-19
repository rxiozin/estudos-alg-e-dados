# Sistema de Busca Otimizado para Catálogo de Produtos - MegaStore

## Descrição do Projeto
Este projeto implementa um sistema de busca e recomendação otimizado para o catálogo de produtos da MegaStore. O sistema permite buscar produtos rapidamente por ID e recomendar itens relacionados com base em conexões como "frequentemente comprados juntos". O objetivo é demonstrar o uso de estruturas de dados eficientes e algoritmos em Rust, garantindo simplicidade, segurança e potencial de escalabilidade para uma loja virtual.

**Funcionalidades principais:**
- Busca de produtos por ID em tempo constante (O(1)).
- Recomendações baseadas em relações usando grafos e Busca em Largura (BFS).
- Validação de entrada e controle de acesso básico.

## Tecnologias Utilizadas
- **Rust**: Linguagem principal, escolhida por sua segurança de memória e desempenho.
- **Cargo**: Ferramenta de build e gerenciamento de dependências do Rust.
- **Estruturas de Dados**: 
  - HashMaps (`std::collections::HashMap`) para busca rápida e representação do grafo.
  - VecDeque (`std::collections::VecDeque`) para a fila do BFS.
- **Algoritmos**: Busca direta em HashMap e BFS para exploração do grafo.
- **Crates**: Nenhuma biblioteca externa foi usada na implementação básica. O crate **Tokio** foi considerado para escalabilidade horizontal (comunicação assíncrona entre nós), mas não implementado.
- **Ferramentas de Teste**: Cargo para compilação e execução; testes unitários não foram implementados.

## Instruções de Como Executar o Sistema
1. Instale o Rust a partir de [https://www.rust-lang.org/](https://www.rust-lang.org/).
2. Clone ou navegue até o diretório do projeto (`MegaStore-Recommender`).
3. No terminal, execute:
   ```bash
   cargo run