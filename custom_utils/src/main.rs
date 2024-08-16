use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{PathBuf, Path};
use polars::prelude::*;

fn main() {
    let my_headers = make_header(); 
    println!("With text:\n{:?}", my_headers);



    let base_dir = Path::new("D:/DevsBE/undergrad_thesis/UndergradThesis/awk_grepping");
    let experiment_name = "run_attention_spatio_temporal";
    
    let _my_dirs = &get_dirs(base_dir, experiment_name);
    println!("\nList of what inside folders\n{:?}", _my_dirs);
    

    // let CsvReadOptions { has_header: false, .. } = my_csv_reader; 
    for d in _my_dirs {
        for dirs in d {
            println!("\n{:?}", dirs);
            let paths = fs::read_dir(dirs).unwrap().map(|res| res.unwrap().path()).collect::<Vec<_>>();
            // println!("{:?}", paths);
            for path in paths {
                let lf = CsvReadOptions::default()
                    .with_has_header(false)
                    .try_into_reader_with_file_path(Some(path.clone()))
                    .unwrap()
                    .finish()
                    .unwrap();
                println!("Name: {}", path.display());
                // let df = rename_columns(&lf, my_headers);
                println!("{:?}", lf);// &lf.expect("REASON").clone());
            }
        }
    }
}

//fn rename_columns(df: &DataFrame, headers: Vec<String>) -> Result<DataFrame> {
//    let new_names: Vec<_> = df.get_column_names().iter().zip(headers).collect();
//    df.set_column_names(headers.unwrap())   
//}


fn make_header() -> Vec<String> {
    let file = File::open("D:/DevsBE/undergrad_thesis/UndergradThesis/awk_grepping/header_datum_wise_augment.txt").expect("Could not open header file");

    let mut reader = BufReader::new(file);
    let mut headers = String::new();
    reader.read_line(&mut headers).expect("coudln't reead the header line");

    headers.trim().split_whitespace().map(String::from).collect()
}

fn get_dirs(base_dir: &Path, name: &str) -> std::io::Result<Vec<PathBuf>> {
    let mut dirs = Vec::new();
            
    for entry in fs::read_dir(base_dir)? {
        let entry = entry?;
        let path = entry.path();
                                        
        // Check if the entry is a directory and contains the `name` substring
        if path.is_dir() &&  path.to_str().unwrap_or("").contains(name) {
            dirs.push(path);
        }
    }

    Ok(dirs)
}
