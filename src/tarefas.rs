use serde::{Serialize, Deserialize};
use chrono::{NaiveDate, Local};

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub enum Status {
    Pendente,
    Concluida,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Tarefa {
    pub titulo: String,
    pub status: Status,
    pub data_conclusao: Option<NaiveDate>,
}

impl Tarefa {
    pub fn nova(titulo: String) -> Self {
        Self {
            titulo,
            status: Status::Pendente,
            data_conclusao: None,
        }
    }

    pub fn esta_concluida(&self) -> bool {
        self.status == Status::Concluida
    }
}
