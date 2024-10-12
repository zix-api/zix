use clap::{Arg, Command};
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
struct ApiSchema {
    name: String,
    version: String,
    endpoints: Vec<Endpoint>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Endpoint {
    path: String,
    method: String,
    request_format: String,
    response_format: String,
}

fn main() {
    let matches = Command::new("Zix CLI")
        .version("1.0")
        .author("Your Name <you@example.com>")
        .about("API Schema Manager")
        .subcommand(
            Command::new("create")
                .about("Create a new API schema")
                .arg(Arg::new("name").required(true))
                .arg(Arg::new("version").required(true))
                .arg(Arg::new("endpoints").required(false)),
        )
        .subcommand(Command::new("list").about("List all created API schemas"))
        .subcommand(
            Command::new("generate-docs")
                .about("Generate documentation for an API schema")
                .arg(Arg::new("name").required(true)),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("create", create)) => {
            let name = create.get_one::<String>("name").unwrap();
            let version = create.get_one::<String>("version").unwrap();
            let endpoints: clap::parser::ValuesRef<'_, String> =
                create.get_many::<String>("endpoints").unwrap_or_default();

            let api_schema: ApiSchema = ApiSchema {
                name: name.clone(),
                version: version.clone(),
                endpoints: endpoints
                    .map(|endpoint| {
                        let parts: Vec<&str> = endpoint.split(',').collect();
                        Endpoint {
                            path: parts.first().unwrap_or(&"").to_string(),
                            method: parts.get(1).unwrap_or(&"GET").to_string(),
                            request_format: parts.get(2).unwrap_or(&"").to_string(),
                            response_format: parts.get(3).unwrap_or(&"").to_string(),
                        }
                    })
                    .collect(),
            };

            let json = serde_json::to_string_pretty(&api_schema).unwrap();
            fs::write(format!("{}.json", name), json).unwrap();
            println!("Created schema: {}.json", name);
        }
        Some(("list", _)) => {
            let paths: fs::ReadDir = fs::read_dir(".").unwrap();
            for path in paths {
                let path: std::path::PathBuf = path.unwrap().path();
                if path.extension().map(|ext: &std::ffi::OsStr| ext == "json").unwrap_or(false) {
                    println!("{}", path.display());
                }
            }
        }
        Some(("generate-docs", generate_docs)) => {
            let name: &String = generate_docs.get_one::<String>("name").unwrap();
            let file_path: String = format!("{}.json", name);
            if Path::new(&file_path).exists() {
                let content: String = fs::read_to_string(file_path).unwrap();
                let api_schema: ApiSchema = serde_json::from_str(&content).unwrap();
                generate_markdown_docs(&api_schema);
            } else {
                println!("Schema file {} not found.", file_path);
            }
        }
        _ => {}
    }
}

fn generate_markdown_docs(schema: &ApiSchema) {
    let mut docs: String = format!("# API Documentation for {}\n\n", schema.name);
    docs.push_str(&format!("## Version: {}\n\n", schema.version));
    docs.push_str("## Endpoints\n\n");

    for endpoint in &schema.endpoints {
        docs.push_str(&format!(
            "- **Path**: {}\n  **Method**: {}\n  **Request Format**: {}\n  **Response Format**: {}\n\n",
            endpoint.path, endpoint.method, endpoint.request_format, endpoint.response_format
        ));
    }

    let docs_file_name: String = format!("{}_docs.md", schema.name);
    let mut file: File = File::create(docs_file_name).expect("Could not create docs file");
    file.write_all(docs.as_bytes()).expect("Could not write to docs file");
    println!("Documentation generated: {}_docs.md", schema.name);
}