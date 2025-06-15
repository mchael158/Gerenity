use crate::{tarefas::{Tarefa, Status}, persistencias};
use chrono::Local;
use eframe::egui;
use egui_plot::{Plot, BarChart, Bar};

pub struct MeuApp {
    tarefas: Vec<Tarefa>,
    nova_tarefa: String,
    tema_escuro: bool,
}

impl MeuApp {
    pub fn new() -> Self {
        Self {
            tarefas: persistencias::carregar(),
            nova_tarefa: String::new(),
            tema_escuro: true,
        }
    }

    fn progresso(&self) -> f32 {
        let total = self.tarefas.len() as f32;
        if total == 0.0 {
            0.0
        } else {
            let concluidas = self.tarefas.iter().filter(|t| t.esta_concluida()).count() as f32;
            concluidas / total
        }
    }

    fn produtividade_por_dia(&self) -> BarChart {
        use std::collections::BTreeMap;
        let mut mapa = BTreeMap::new();

        for tarefa in &self.tarefas {
            if let Some(data) = tarefa.data_conclusao {
                *mapa.entry(data).or_insert(0) += 1;
            }
        }

        let barras: Vec<Bar> = mapa
            .into_iter()
            .enumerate()
            .map(|(i, (_data, count))| Bar::new(i as f64, count as f64))
            .collect();

            BarChart::new("Produtividade", barras).width(0.6)
    }
}

impl eframe::App for MeuApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Tema claro/escuro
        if self.tema_escuro {
            ctx.set_visuals(egui::Visuals::dark());
        } else {
            ctx.set_visuals(egui::Visuals::light());
        }

        egui::TopBottomPanel::top("top").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("🌗 Tema:");
                if ui.button("Alternar").clicked() {
                    self.tema_escuro = !self.tema_escuro;
                }

                if ui.button("🧹 Limpar tudo").clicked() {
                    self.tarefas.clear();
                    persistencias::salvar(&self.tarefas);
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("📝 Gerenciador de Tarefas");

            ui.horizontal(|ui| {
                ui.text_edit_singleline(&mut self.nova_tarefa);
                if ui.button("Adicionar").clicked() && !self.nova_tarefa.trim().is_empty() {
                    self.tarefas.push(Tarefa::nova(self.nova_tarefa.trim().to_string()));
                    self.nova_tarefa.clear();
                    persistencias::salvar(&self.tarefas);
                }
            });

            ui.separator();

            let mut salvar_necessario = false;
            let mut remover = None;

            for (i, tarefa) in self.tarefas.iter_mut().enumerate() {
                ui.horizontal(|ui| {
                    let mut concluida = tarefa.esta_concluida();
                    if ui.checkbox(&mut concluida, "").changed() {
                        tarefa.status = if concluida {
                            tarefa.data_conclusao = Some(Local::now().date_naive());
                            Status::Concluida
                        } else {
                            tarefa.data_conclusao = None;
                            Status::Pendente
                        };
                        salvar_necessario = true;
                    }

                    let texto = match tarefa.status {
                        Status::Concluida => format!("✅ {}", tarefa.titulo),
                        Status::Pendente => format!("⬜ {}", tarefa.titulo),
                    };
                    ui.label(texto);

                    if ui.button("🗑").clicked() {
                        remover = Some(i);
                    }
                });
            }

            if let Some(i) = remover {
                self.tarefas.remove(i);
                salvar_necessario = true;
            }

            if salvar_necessario {
                persistencias::salvar(&self.tarefas);
            }

            ui.separator();
            ui.label(format!("Progresso: {:.0}%", self.progresso() * 100.0));
            ui.add(egui::widgets::ProgressBar::new(self.progresso()).show_percentage());

            ui.separator();
            ui.heading("📊 Produtividade");
            Plot::new("produtividade").height(200.0).show(ui, |plot_ui| {
                plot_ui.bar_chart(self.produtividade_por_dia());
            });
        });
    }
}
