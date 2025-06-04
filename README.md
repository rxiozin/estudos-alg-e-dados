
# Sistema de Busca Otimizado para Catálogo de Produtos - MegaStore

## Descrição do Projeto

Este projeto implementa um sistema de busca e recomendação otimizado para o catálogo de produtos da MegaStore. O sistema permite buscar produtos rapidamente por ID e recomendar itens relacionados com base em conexões como "frequentemente comprados juntos".  

O objetivo é demonstrar o uso de estruturas de dados eficientes e algoritmos em Rust, garantindo simplicidade, segurança e potencial de escalabilidade para uma loja virtual.

## Funcionalidades principais

- ✅ Busca de produtos por ID em tempo constante **O(1)** usando **HashMap**.
- ✅ Recomendações baseadas em relações entre produtos, utilizando **grafos** e algoritmo de **Busca em Largura (BFS)**.
- ✅ Validação de entrada e controle de acesso básico via chave de segurança.
- ✅ Sistema modularizado com separação clara entre catálogo, busca, modelo de dados e recomendação.

---

## Tecnologias Utilizadas

- **Rust**: Linguagem principal, escolhida por sua segurança de memória e alto desempenho.
- **Cargo**: Ferramenta oficial de build e gerenciamento de dependências do Rust.

### Estruturas de Dados:
- **HashMap** (`std::collections::HashMap`): Para busca rápida de produtos por ID e representação das relações no grafo.
- **VecDeque** (`std::collections::VecDeque`): Fila utilizada na execução da Busca em Largura (BFS).

### Algoritmos:
- **Busca direta** via HashMap.
- **Busca em Largura (BFS)** para explorar relações e gerar recomendações.

### Crates:
- Nenhuma biblioteca externa foi utilizada na implementação básica.
- O crate **Tokio** foi considerado para futuras implementações de escalabilidade e comunicação assíncrona, mas **não foi implementado**.

---

## Instruções de Como Executar o Sistema

1. **Instale o Rust** (se ainda não o fez):  
   [https://www.rust-lang.org/](https://www.rust-lang.org/)

2. **Clone o repositório** ou navegue até o diretório do projeto:

```bash
git clone <URL-do-repositório>
cd MegaStore-Recommender
```

3. **Compile e execute o sistema** com o comando:

```bash
cargo run
```

O sistema realizará automaticamente exemplos de busca e recomendação, conforme definidos no `main.rs`.

---

## Instruções de Como Executar os Testes

> ⚠️ **Nota:** Atualmente, o projeto **não possui testes unitários implementados**.  

Para compilar e verificar o código:  

```bash
cargo build
```

Para rodar eventuais testes futuros:  

```bash
cargo test
```

---

## Exemplos de Uso

### ➡️ Exemplo 1: Busca de Produto por ID

```rust
match buscar_produto(&catalogo, 1, "senha_secreta") {
    Some(produto) => println!("Produto encontrado: {:?}", produto),
    None => println!("Busca falhou: ID não encontrado ou chave inválida"),
}
```

**Saída esperada:**

```
Produto encontrado: Produto { id: 1, nome: "Camiseta", preco: 29.99, categoria: "Roupas" }
```

---

### ➡️ Exemplo 2: Recomendação de Produtos Relacionados

```rust
let recomendacoes = recomendar_produtos(&grafo, &catalogo, 1, 3);
println!("Recomendações: {:?}", recomendacoes);
```

**Saída esperada:**

```
Recomendações: [Produto { id: 2, nome: "Calça", ... }, Produto { id: 3, nome: "Tênis", ... }]
```

---

## Arquitetura do Sistema

O sistema está organizado em módulos independentes:

- **modelo.rs**: Define a estrutura `Produto` e o tipo `Grafo`.
- **catalogo.rs**: Cria e popula o catálogo de produtos, bem como o grafo de relações.
- **busca.rs**: Implementa a busca de produto por ID com controle de acesso.
- **recomendacao.rs**: Implementa o algoritmo de recomendação usando BFS.
- **main.rs**: Ponto de entrada do programa, realizando exemplos de busca e recomendação.

---

## Algoritmos e Estruturas de Dados Utilizados

- **Tabelas Hash (HashMap)**: Permitem busca de produtos em tempo constante `O(1)`.
- **Grafo não ponderado**: Representa relações entre produtos.
- **Busca em Largura (BFS)**: Explora o grafo para gerar recomendações sem repetir produtos.
- **Fila (VecDeque)**: Utilizada no BFS para gerenciamento eficiente de nós a serem visitados.

---

## Considerações sobre Desempenho e Escalabilidade

- **Desempenho**: A busca por produto é realizada em tempo constante `O(1)`, garantindo rapidez mesmo com grandes catálogos.
- **Recomendações**: A BFS garante que as recomendações sejam feitas de forma eficiente, explorando conexões próximas no grafo.
- **Escalabilidade**:  
  - A arquitetura modular facilita a expansão para novas funcionalidades.  
  - O uso potencial de crates como **Tokio** pode permitir a escalabilidade horizontal com comunicação assíncrona.  

**Testes de desempenho** não foram formalmente implementados, mas o uso de estruturas eficientes garante boa performance para catálogos de médio porte.

---

## Contribuições

Este projeto é acadêmico e está aberto a contribuições!  
Para contribuir:  

1. Faça um fork do repositório.  
2. Crie uma branch para sua feature ou correção (`git checkout -b minha-feature`).  
3. Commit suas mudanças (`git commit -m 'Minha feature'`).  
4. Faça push para sua branch (`git push origin minha-feature`).  
5. Abra um **Pull Request**.

---

## Licença

Este projeto está licenciado sob a **MIT License**.  
Consulte o arquivo `LICENSE` para mais detalhes.

---

## Autor

MegaStore Recommender — Projeto acadêmico de sistemas de busca em Rust.
