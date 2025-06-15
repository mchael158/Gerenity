use crate::tarefas::Tarefa;
use std::fs;
use std::path::PathBuf;

fn caminho() -> PathBuf {
    let mut path = std::env::current_dir().unwrap_or_else(|_| ".".into());
    path.push("tarefas.json");
    path
}

pub fn salvar(tarefas: &Vec<Tarefa>) {
    if let Ok(json) = serde_json::to_string_pretty(tarefas) {
        let _ = fs::write(caminho(), json);
    }
}

pub fn carregar() -> Vec<Tarefa> {
    let path = caminho();
    if path.exists() {
        if let Ok(content) = fs::read_to_string(path) {
            if let Ok(tarefas) = serde_json::from_str(&content) {
                return tarefas;
            }
        }
    }
    Vec::new()
}
