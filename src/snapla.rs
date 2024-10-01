use eframe::glow::LEFT;
use egui::{emath::smart_aim, load, Button, Layout, ScrollArea, Ui};
use fxhash::FxHashMap;
use serde::{Deserialize, Serialize};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
pub(crate) struct Snapla {
    // Example stuff:
    pub(crate) label: String,

    pub(crate) value: f32,
    pub(crate) data: Daten,
    pub(crate) selected_lms: Vec<(String, u16)>,
    pub(crate) new_selection: Option<String>,
    pub(crate) view: View,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum View {
    View1,
    View2,
    View3,
}

impl Snapla {
    /// Called once before the first frame.
    pub(crate) fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.
        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        let data = Self::load_data();
        _cc.egui_ctx.set_visuals(egui::Visuals::dark());
        Snapla {
            label: "Hello, world!".to_owned(),
            value: 5.0,
            data: data,
            view: View::View1,
            selected_lms: Vec::new(),
            new_selection: None,
        }
    }

    fn calculate_nutrition(&self) -> FxHashMap<String, f64> {
        let mut result = FxHashMap::default();
        for (lm, amount) in self.selected_lms.iter() {
            for (nutrient, concentration) in self.data.lebensmittel.get(lm).unwrap().iter() {
                let value = result.entry(nutrient.clone()).or_insert(0.0);
                *value += concentration * (*amount as f64);
            }
        }
        result
    }

    fn load_data() -> Daten {
        let bytes = include_bytes!("../info_parser_from_docx/serialized/daten.json");
        let data: Daten = serde_json::from_slice(bytes).unwrap();
        for (key, value) in data.lebensmittel.iter() {
            println!("\n{}: ", key);
            for (key, value) in value.iter() {
                println!("{}: {}", key, value);
            }
        }
        for tagesbedarf in data.tagesbedarf.iter() {
            println!("{}: {}", tagesbedarf.name, tagesbedarf.wert);
        }
        data
    }

    pub(crate) fn plan(&mut self, ui: &mut egui::Ui) {
        if self.new_selection.is_some() {
            let refer = self.new_selection.as_ref().unwrap().clone();
            self.selected_lms.push((refer, 0));
            self.new_selection = None;
        }
        if ui.button("Test").clicked() {
            println!("Test");
        }
        let mut to_remove = None;

        ui.ctx().style_mut(|style| {
            style.spacing.slider_width = 200.0;
            
        });

        ui.vertical_centered(|ui| {
            ui.horizontal(|ui| {
                egui::Grid::new("LMS").show(ui, |ui| {
                    let len = self.selected_lms.len();
                    for i in 0..len {
                        let name = self.selected_lms[i].0.clone();
                        ui.label(format!("{}", &name));
                        
                        if ui.button("Entfernen").clicked() {
                            to_remove = Some(i);
                        }
                        ui.add_space(60.0);
                        ui.add(
                            egui::Slider::new(&mut self.selected_lms[i].1, 0..=500)
                                .text("g   ".to_owned() + &name + "").step_by(5.0),
                        );
                        ui.end_row();
                    }
                })
            })
        });

        if let Some(i) = to_remove {
            self.selected_lms.remove(i);
        }

        egui::ComboBox::from_label("Zusätzliches Lebensmittel")
            .selected_text(format!("{:?}", "Lebensmittel hinzufügen"))
            .show_ui(ui, |ui| {
                self.data
                    .lebensmittel
                    .iter()
                    .filter(|x| !self.selected_lms.iter().any(|y| &y.0 == x.0))
                    .for_each(|(key, _value)| {
                        ui.selectable_value(&mut self.new_selection, Some(key.to_owned()), key);
                    });
            });
    }

    pub(crate) fn show_result(&mut self, ui: &mut egui::Ui) {}

    pub(crate) fn show_food_details(&mut self, ui: &mut egui::Ui) {
        let mut style = (*ui.ctx().style()).clone();
        style.spacing.item_spacing = egui::vec2(10.0, 2.0);
        ui.ctx().set_style(style);
        ui.vertical_centered(|ui| {
            ui.horizontal(|ui| {
                egui::Grid::new("LMS").show(ui, |ui| {
                    for (key, value) in self.data.lebensmittel.iter() {
                        ui.label(format!("{}", key));
                        ui.label(format!("{:?}", value));
                        ui.end_row();
                    }
                })
            })
        });
    }
}

#[derive(Debug, Deserialize)]
pub(crate) struct TagesBedarfInMg {
    pub(crate) name: String,
    pub(crate) wert: f64,
}

pub(crate) type LMInformation = FxHashMap<String, LMConcentration>;
pub(crate) type LM = FxHashMap<String, LMInformation>;
pub(crate) type LMConcentration = f64;

#[derive(Deserialize)]
pub(crate) struct Daten {
    pub(crate) lebensmittel: FxHashMap<String, LMInformation>,
    pub(crate) tagesbedarf: Vec<TagesBedarfInMg>,
}
