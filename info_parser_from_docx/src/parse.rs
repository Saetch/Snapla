
use fxhash::FxHashMap;
use docx_rust::document::{BodyContent, ParagraphContent, RunContent};
use docx_rust::DocxFile;
use crate::{ TagesBedarfInMg, ParsedContent};

pub(super) fn read_parse_whacky_input_and_deserialize_docx_file(filepath: &str) -> (ParsedContent, TagesBedarfInMg) {
    let docx = DocxFile::from_file(filepath).unwrap();
    let docx = docx.parse().unwrap();
    let content = docx.document.body.content;
    let mut content_string = String::new();
    for paragraph in &content {
        let par_content = match paragraph {
            BodyContent::Paragraph(p) => p,
            _ => continue,
        };
        for pr_cnt in &par_content.content {
            match pr_cnt {
                ParagraphContent::Run(r) => {
                    for content in &r.content {
                        match content {
                            RunContent::Text(t) => {
                                content_string.push_str(&t.text);
                                content_string.push_str(" ");
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }
    }

    let (parsed_content, nutr_value) = deserialize_content(&content_string);

    (parsed_content, nutr_value)

}




fn deserialize_content(content: &str) -> (ParsedContent, TagesBedarfInMg) {
    let lms = FxHashMap::default();

    let mut parts = content.split(" ");
    let mut initialized = false;
    let mut requirement_name = String::new();
    let mut required_amount: f64 = 0.0;

    let mut rt = ParsedContent {
        lms: lms
    };
    let mut searching_for_name = true;
    let mut still_at_the_start = true;
    let mut current_lm_name = String::new();
    let mut current_lm_concentration_in_g: u32 = 0;
    let mut current_lm_relative_weight: Option<f64> = None;
    let mut expect_g_after_conc = true;
    let mut expect_mg_after_rel = true;
    let mut mg_expected = 2;
    let mut built_string = String::new();
    let mut carry_over_string = String::new();
    println!("\n\n\n\n");
    while let Some(part) = parts.next(){
        if part == ""{
            println!("Skipping: {}", part);
            continue;
        }
        if !initialized{

            if requirement_name == "" || searching_for_name {
                if part == "Bedarf" {
                    searching_for_name = false;
                    continue;
                }
                println!("Name: {}", part);
                requirement_name.push_str(part);
                if requirement_name.ends_with("Bedarf") {
                    requirement_name = requirement_name.replace("Bedarf", "");
                    searching_for_name = false;
                    continue;
                }
                continue;
            }
            if required_amount == 0.0 {

                println!("Required amount: {}", part);
                let im: Vec<&str> = part.split("-").collect();
                let result = im[0].replace(",", ".").parse();
                if result.is_err(){
                    requirement_name.push_str(" ");
                    requirement_name.push_str(part);
                    continue;
                }
                required_amount = result.unwrap();
                continue;
            }
            if part == "mg" {
                mg_expected -= 1;
                if mg_expected == 0 {
                    requirement_name = requirement_name.replace("Vitamin", "Vitamin ");
                    initialized = true;
                }
            }
            continue;
        }

        if current_lm_name == "" || still_at_the_start{
            let parse_try = part.parse::<f64>();
            if parse_try.is_ok(){
                if !still_at_the_start {
                    println!("LM Name: {}", part);
                    current_lm_name = carry_over_string.clone();
                }
                still_at_the_start = false;
            }else{
                println!("LM Name: {}", part);
                current_lm_name = part.to_string();
                carry_over_string = String::new();
                continue;
            }
            carry_over_string = String::new();
        }
        if current_lm_concentration_in_g == 0 {
            println!("current amount: {}", part);
            let mut to_parse = part.to_string();
            if part.ends_with("g"){
                to_parse = part.replace("g","");
                let result = to_parse.parse();
                if result.is_err(){
                    current_lm_name.push_str(part);
                    continue;
                }
                current_lm_concentration_in_g = result.unwrap();
                expect_g_after_conc = false;
                continue;
            }else{
                let result = to_parse.parse();
                if result.is_err(){
                    current_lm_name.push_str(part);
                    continue;
                }
                current_lm_concentration_in_g = result.unwrap();
                continue;
            }

        }
        if expect_g_after_conc {
            expect_g_after_conc = false;
            if part == "g" {
                continue;
            }else{
                println!("WARNING: Expected g after concentration, but got: {}", part);
            }
        }
        if current_lm_relative_weight == None {
            let mut part_string = part.to_string();
            println!("Weight in amount: {}", part);
            if part.contains("mg"){
                part_string = part_string.replace("mg","").parse().unwrap();
                expect_mg_after_rel = false;
            }
            built_string.push_str(part);
            if part_string.contains(","){
                current_lm_relative_weight = Some(part_string.replace(",",".").parse().unwrap());
            }else{
                current_lm_relative_weight = Some(part_string.parse().unwrap());
            }
            if expect_mg_after_rel {
                continue;
            }
        }
        if expect_mg_after_rel {
            if part != "mg"  {
                let mut currently_number :Option<f64> = None;
                let currently_number_res = built_string.clone().replace(",", ".").parse();
                if currently_number_res.is_ok(){
                    currently_number = Some(currently_number_res.unwrap());
                }
                built_string.push_str(part);
                if built_string.ends_with("mg") || built_string.ends_with("g"){
                    println!("Parsing: {}", built_string);
                    current_lm_relative_weight = Some(built_string.replace(",",".").replace("m", "").replace("g", "").parse().unwrap());

                }else if let Some(number) = currently_number {
                    let result = built_string.replace(",", ".").parse::<f64>();
                    if result.is_err() && !(built_string.ends_with(".") || built_string.ends_with(",")){
                        carry_over_string.push_str(part);
                        current_lm_relative_weight = Some(number);
                    }else{
                        continue;
                    }
                }else {
                    continue;
                }
            }else{
                assert!(part=="mg" || part=="g");
                println!("Weight in mg: {}", built_string);
                current_lm_relative_weight = Some(built_string.replace(",",".").replace("mg", "").parse().unwrap());
                println!("Weight in mg: {}", current_lm_relative_weight.unwrap() as f64);
            }

        }
        let displayed_amount_in_mg = current_lm_relative_weight.unwrap();

        let mg_per_g = if displayed_amount_in_mg == 0.0 {0.0} else { displayed_amount_in_mg / current_lm_concentration_in_g as f64};
        let mut this_conc = FxHashMap::default();
        this_conc.insert(requirement_name.to_string(), mg_per_g);

        rt.lms.insert(current_lm_name.to_string(), this_conc);
        current_lm_name = String::new();
        current_lm_concentration_in_g = 0;
        current_lm_relative_weight = None;
        expect_g_after_conc = true;
        expect_mg_after_rel = true;
        built_string = String::new();
    }

    (rt, TagesBedarfInMg{name: requirement_name, wert: required_amount as f64})
}