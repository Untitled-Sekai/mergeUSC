use clap::{Parser, ArgAction};
use mergeusc_core::{UscFile, UscMerger};
use std::fs;
use std::path::PathBuf;
use std::error::Error;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    // input
    #[arg(required = true)]
    input_files: Vec<PathBuf>,

    // output 
    #[arg(short, long)]
    output: Option<PathBuf>,

    /// detail
    #[arg(short, long, action = ArgAction::SetTrue)]
    verbose: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    
    if args.verbose {
        println!("launching MergeUSC...");
    }
    
    let mut usc_files = Vec::new();
    for input_path in &args.input_files {
        if args.verbose {
            println!("load file: {}", input_path.display());
        }
        
        let content = fs::read_to_string(input_path)
            .map_err(|e| format!("can't read「{}」: {}", input_path.display(), e))?;
            
        let usc_file: UscFile = serde_json::from_str(&content)
            .map_err(|e| format!("failed parse「{}」: {}", input_path.display(), e))?;
            
        usc_files.push(usc_file);
    }
    
    if args.verbose {
        println!("Merge {} usc file...", usc_files.len());
    }
    
    let merged = UscMerger::merge(usc_files)?;
    
    let merged_json = serde_json::to_string_pretty(&merged)?;
    
    match &args.output {
        Some(output_path) => {
            fs::write(output_path, merged_json)?;
            if args.verbose {
                println!("saved {}", output_path.display());
            }
        },
        None => {
            println!("{}", merged_json);
        }
    }
    
    if args.verbose {
        println!("completed successfully.");
    }
    
    Ok(())
}