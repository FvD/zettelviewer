use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use pulldown_cmark::{Parser, html, Options, Event, Tag, HeadingLevel};
use warp::Filter;

// Store both HTML content and the title
struct MarkdownInfo {
    title: String,
    html_content: String,
}

type HtmlCache = Arc<Mutex<HashMap<String, MarkdownInfo>>>;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: render <folder_with_markdown>");
        std::process::exit(1);
    }
    
    let folder_path = &args[1];
    let html_cache = Arc::new(Mutex::new(HashMap::new()));
    
    // Load all markdown files into memory
    if let Err(e) = load_markdown_files(folder_path, html_cache.clone()) {
        eprintln!("Error loading markdown files: {}", e);
        std::process::exit(1);
    }
    
    // Define the routes
    let cache = html_cache.clone();
    let files = warp::path::param()
        .map(move |name: String| {
            let cache = cache.lock().unwrap();
            match cache.get(&name) {
                Some(info) => warp::reply::html(info.html_content.clone()),
                None => warp::reply::html(format!("<h1>File not found: {}</h1>", name))
            }
        });
        
    let index = warp::path::end().map(move || {
        let cache = html_cache.lock().unwrap();
        let file_list = cache.iter()
            .map(|(name, info)| {
                format!("<li><a href=\"/{}\">{} - {}</a></li>", 
                    name, 
                    name, 
                    if info.title.is_empty() { "[No title]" } else { &info.title }
                )
            })
            .collect::<Vec<_>>()
            .join("\n");
            
        warp::reply::html(format!(
            "<!DOCTYPE html>\n<html>\n<head>\n<title>Markdown Files</title>\n\
            <style>\nbody {{ font-family: Arial, sans-serif; line-height: 1.6; padding: 20px; max-width: 800px; margin: 0 auto; }}\n\
            li {{ margin-bottom: 8px; }}\n\
            </style>\n</head>\n<body>\n<h1>Available Files</h1>\n<ul>\n{}\n</ul>\n</body>\n</html>",
            file_list
        ))
    });
    
    println!("Server started at http://localhost:3030");
    println!("Press Ctrl+C to stop");
    
    warp::serve(index.or(files))
        .run(([127, 0, 0, 1], 3030))
        .await;
}

fn load_markdown_files(folder_path: &str, html_cache: HtmlCache) -> std::io::Result<()> {
    let path = Path::new(folder_path);
    
    if !path.is_dir() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            format!("Error: {} is not a directory", folder_path)
        ));
    }
    
    let mut cache = html_cache.lock().unwrap();
    
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_file() && path.extension().map_or(false, |ext| ext == "md") {
            let file_name = path.file_stem().unwrap().to_string_lossy().to_string();
            let (title, html_content) = parse_markdown_file(&path)?;
            
            cache.insert(file_name, MarkdownInfo { 
                title, 
                html_content 
            });
            
            println!("Loaded: {}", path.display());
        }
    }
    
    println!("All Markdown files loaded successfully.");
    Ok(())
}

fn parse_markdown_file(file_path: &PathBuf) -> std::io::Result<(String, String)> {
    // Read markdown content
    let markdown_content = fs::read_to_string(file_path)?;
    
    // Parse markdown
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);
    
    // First pass to extract title
    let parser = Parser::new_ext(&markdown_content, options.clone());
    let mut title = String::new();
    let mut in_heading = false;
    
    for event in parser {
        match event {
            Event::Start(Tag::Heading(HeadingLevel::H1, _, _)) => {
                in_heading = true;
            },
            Event::Text(text) if in_heading => {
                title.push_str(&text);
            },
            Event::End(Tag::Heading(HeadingLevel::H1, _, _)) => {
                in_heading = false;
                break; // We only care about the first h1 heading
            },
            _ => {}
        }
    }
    
    // Second pass to generate HTML
    let parser = Parser::new_ext(&markdown_content, options);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    
    // Add basic HTML wrapper
    let html_content = format!(
        "<!DOCTYPE html>\n<html>\n<head>\n<meta charset=\"UTF-8\">\n<title>{}</title>\n\
        <style>\nbody {{ font-family: Arial, sans-serif; line-height: 1.6; padding: 20px; max-width: 800px; margin: 0 auto; }}\n\
        pre {{ background-color: #f4f4f4; padding: 12px; border-radius: 4px; overflow-x: auto; }}\n\
        code {{ background-color: #f4f4f4; padding: 2px 4px; border-radius: 4px; }}\n\
        </style>\n</head>\n<body>\n{}\n</body>\n</html>",
        if title.is_empty() { file_path.file_stem().unwrap().to_string_lossy().to_string() } else { title.clone() },
        html_output
    );
    
    Ok((title, html_content))
}
