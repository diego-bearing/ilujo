use std::env;
use std::io::Write;
use std::{fs, io::Read};
use std::path::Path;

const TARGET_DIR: &'static str = "";
const PLACEHOLDER_NAME: &'static str = "{{component_name}}";

fn create_component_boilerplate(component_name: &str) {
    // read dir
    let current_dir = env::current_dir().expect("and the current dir?");
    let mut exe_dir = env::current_exe().expect("Fails getting exe path");
    exe_dir.pop();

    let target_dir = current_dir.join(TARGET_DIR);

    let dir_entries = fs::read_dir(&target_dir).expect("should read the target dir");
    // check if dir exist
    dir_entries.into_iter().for_each(|entry| {
        let path = entry.expect("Failed reading dir entries").path();

        if path.ends_with(component_name) {
            panic!("Directory {component_name} already exists!");
        }        
    });

    let component_dir_path = target_dir.join(component_name);
    fs::create_dir(&component_dir_path).expect("Fails creating component dir");
    println!("Directory {component_dir_path:?} created");
    
    let templates = [
        ("component.tmpl", "{{component_name}}.tsx"),
        ("types.tmpl", "{{component_name}}.types.ts"),
        ("stories.tmpl", "{{component_name}}.stories.tsx"),
        ("index.tmpl", "index.ts"),
    ];

    for (name, filename_tmpl) in templates {
        let path = Path::new(&exe_dir).join("templates").join(name);
        let mut file = fs::File::open(&path).expect("Fails opening template file");
        let mut text = String::new();
        file.read_to_string(&mut text).expect("Fails reading the template file");
        let replaced = text.replace(PLACEHOLDER_NAME, component_name);
        let file_name = String::from(filename_tmpl).replace(PLACEHOLDER_NAME, component_name);
        
        let destination_path = &component_dir_path.join(&file_name);
        
        let mut destination_file = fs::File::create(destination_path).expect("Fails creating the destination file");
        destination_file.write_all(replaced.as_bytes()).expect("Fails writing the destination file");
        println!("File {path:?} created");
    }

    let index_path = Path::new(&target_dir).join("index.ts");

    match fs::File::options().append(true).open(&index_path) {
        Err(error) => {
            println!("opening index: {index_path:?}: {error}");
        },
        Ok(mut index_file) => {
            let export_statement = format!("export * from './{component_name}'\n");
            index_file.write(export_statement.as_bytes()).expect("Fails updating the index.ts");
            println!("File {index_path:?} updated");
        }
    };
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let component_name = args.get(1).unwrap_or_else(|| panic!("no entry"));

    create_component_boilerplate(component_name);
}
