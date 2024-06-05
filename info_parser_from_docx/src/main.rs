use std::fs;

use fxhash::FxHashMap;
use parse::read_parse_whacky_input_and_deserialize_docx_file;
use serde::Serialize;

mod parse;


#[derive(Debug)]
pub(crate) struct ParsedContent {
    pub(crate) lms : fxhash::FxHashMap<String, LMInformation>,
}

#[derive(Debug, Serialize)]
pub(crate) struct TagesBedarfInMg {
    pub(crate) name: String,
    pub(crate) wert: f64,
}

pub(crate) type LMInformation = FxHashMap<String, LMConcentration>;

pub(crate) type LMConcentration = f64;

#[derive(Serialize)]
pub(crate) struct Daten {
    pub(crate) lebensmittel: FxHashMap<String, LMInformation>,
    pub(crate) tagesbedarf: Vec<TagesBedarfInMg>,
}

fn main() {
    let mut parsed_contents = Vec::new();
    let mut nut_values = Vec::new();
    let information_folder = fs::read_dir("information").unwrap();
    for file in information_folder {
        let file = file.unwrap();
        let file_name = file.file_name();
        let file_name = file_name.to_str().unwrap();

        println!("{}", file_name);  
        let parsed_content = read_parse_whacky_input_and_deserialize_docx_file(&format!("information/{}", file_name));
        
        for (key, value) in &parsed_content.0.lms {
            println!("Lebensmittel: {}", key);
            for (k, v) in value {
               // println!("{}: {}", k, v)
            }
    
        }
        parsed_contents.push(parsed_content.0);
        nut_values.push(parsed_content.1);
    }
    let combined_info = combine_info(parsed_contents);
    
    let daten = Daten {
        lebensmittel: combined_info,
        tagesbedarf: nut_values,
    };
    let json = serde_json::to_string(&daten).unwrap();
    fs::write("serialized/daten.json", json).unwrap();
}


fn combine_info(parsed_contents: Vec<ParsedContent>) -> FxHashMap<String, LMInformation> {
    let mut combined_info = FxHashMap::default();
    for content in parsed_contents{
        for (k, value) in content.lms {
            let mut key = k;
            if key.starts_with("m"){
                key = key[1..].to_string();
            }
            if key.starts_with("g") && !key.starts_with("ge"){
                key = key[1..].to_string();
            }
            if key.starts_with("?"){
                key = key[1..].to_string();
            }
            if key.ends_with("?"){
                key = key[..key.len()-1].to_string();
            }
            key = key.replace("Vitamin", "Vitamin ");
            
            if combined_info.contains_key(&key){
                let values = &value;
                combined_info.entry(key).and_modify(|map: &mut FxHashMap<String, f64>| map.extend(values.iter().map(|(k, v)| (k.clone(), *v))));
            }
            else {
                combined_info.insert(key, value);
            }
        }
    }
    combined_info
}