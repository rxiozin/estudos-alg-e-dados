#[allow(dead_code)]
#[derive(Debug)]
pub struct Produto {
    pub id: u32,
    pub nome: String,
    pub preco: f32,
    pub categoria: String,
}

pub type Grafo = std::collections::HashMap<u32, Vec<u32>>;