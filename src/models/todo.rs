#[derive(Debug, Clone)]
pub struct Todo {
    pub id: u32,
    pub done: bool,
    pub content: String,
}
