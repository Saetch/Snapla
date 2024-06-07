use egui::load;
use fxhash::FxHashMap;
use serde::{Deserialize, Serialize};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
pub(crate) struct Snapla {
    // Example stuff:
    pub(crate) label: String,

    pub(crate) value: f32,
    pub(crate) inner_state: State,
}

#[derive(Default)]
pub(crate) enum State {
    #[default]
    Empty,
    Data{
        data: Vec<String>,
    }
}

impl Snapla {
    /// Called once before the first frame.
    pub(crate) fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.
        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        Self::load_data();
        _cc.egui_ctx.set_visuals(egui::Visuals::dark());
        Snapla {
            label: "Hello, world!".to_owned(),
            value: 5.0,
            inner_state: State::default(),
        }

    }

    pub(crate) fn data() -> Self{
        Snapla {
            label: "Hello, world!".to_owned(),
            value: 5.0,
            inner_state: State::default(),
        }
    }

    fn load_data() -> State{

        let bytes = include_bytes!("../info_parser_from_docx/serialized/daten.json");
        let data: Daten = serde_json::from_slice(bytes).unwrap();
        for (key, value) in data.lebensmittel.iter(){
            println!("{}: ", key);
            for (key, value) in value.iter(){
                println!("{}: {}", key, value);
            }
        }
        for tagesbedarf in data.tagesbedarf.iter(){
            println!("{}: {}", tagesbedarf.name, tagesbedarf.wert);
        }
        State::Data{
            data: vec!["Hello".to_string(), "World".to_string()]
        }
    }
}



#[derive(Debug, Deserialize)]
pub(crate) struct TagesBedarfInMg {
    pub(crate) name: String,
    pub(crate) wert: f64,
}

pub(crate) type LMInformation = FxHashMap<String, LMConcentration>;

pub(crate) type LMConcentration = f64;

#[derive(Deserialize)]
pub(crate) struct Daten {
    pub(crate) lebensmittel: FxHashMap<String, LMInformation>,
    pub(crate) tagesbedarf: Vec<TagesBedarfInMg>,
}
