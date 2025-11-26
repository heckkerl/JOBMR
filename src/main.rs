use project_web_url::beat_map_downl;
use std::io;
use std::fs::File;
use std::io::Write;
//use serde::Deserialize;
use std::io::{BufRead, BufReader};
use std::ffi::OsStr;
//use toml;
use clap::{Parser, Subcommand};
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "JOBMR")]
#[command(author = "heckkerl")]
#[command(version = "alpha")]
#[command(about = "OSU!!!", long_about = "ngay hom nay ta cung hop hoan noi day")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    /// Path to your osu Songs folder
    #[arg(short = 'p', long,)]
    path:Option<PathBuf>,

    /// Output folders
    #[arg(short = 'o', long)]
    output:Option<PathBuf>,
    
    /// Path to output file (use for download map)
    #[arg(short ='i', long)]
    input:Option<String>,

    /// Cookies 
    #[arg(short = 'c', long)]
    cookies : Option<String>,
}

#[derive(Subcommand)]
enum Commands {
    /// TO BASIC
    How,
    /// print
    Print,
    /// download
    Import,
}
struct Songs {
    Id : String,
    Name:String,
    Link : String,
}
struct MusicLib {
    Songs : Vec<Songs>,
}
fn main() -> std::io::Result<()> {
    let SmpLink1 = "https://osu.ppy.sh/beatmapsets/";let SmpLink2 = "/download";
    let cli = Cli::parse();
    let mut data = MusicLib {
        Songs: Vec::new(),
    };
    let folders: Vec<_> = if let Some(clipath) = &cli.path{
        fs::read_dir(&clipath)?
            .filter_map(|e| {
                let p = e.ok()?.path();
                if p.is_dir() { Some(p) } else { None }
            })
            .collect()
    } else {Vec::new()};
    
    let mut output: Box<dyn Write> = match &cli.output {
        Some(path) => Box::new(File::create(path)?),
        None => Box::new(io::stdout()),
    };
    match cli.command {
        Commands::Print => {
            for fldr in &folders{
                let ten : &OsStr = fldr.file_name().unwrap();
                if let Some(name) = ten.to_str(){
                    let filecode : String = name.chars()
                        .take_while(|c| c.is_ascii_digit())
                        .collect();
                    if !filecode.is_empty() && name.chars().nth(filecode.len()) == Some(' '){
                        let link = format!("{}{}{}",&SmpLink1,&filecode,&SmpLink2);
                        data.Songs.push(Songs{
                            Id: filecode.clone().into(),
                            Name: name.clone().into(),
                            Link : link.clone().into(),
                        });
                        println!("code: {} name : {} \nlink:{}",&filecode, &name, &link);
                        let _ = writeln!(output,"{}{}{}",&SmpLink1,&filecode,&SmpLink2);
                    }
                    else {
                        println!("--- code: -- name : {}",name);
                    }

                }
            }
        },
        Commands::How => {
            println!("JOBMR -p <PATH TO SONGS FOLDER> <COMMANDS>\nExample \"JOBMR -p path/to/my/osu/Songs print\"")
        },
        Commands::Import =>{
            if let Some(ipath) = &cli.input {
                let cokk = match &cli.cookies {
                    Some(_str) => _str,
                    None => {
                        eprintln!("Error cookies");
                        return Ok(());
                    },
                };
                let iFile = File::open(ipath)?;
                let reader = BufReader::new(iFile);
                println!("{}",ipath);
                for line_result in reader.lines() {
                    println!("ok"); // giờ chắc chắn chạy
                    match line_result {
                        Ok(line) => {println!("{}", line.clone());
                            beat_map_downl(cokk.clone(), line);
                        },
                        Err(e) => eprintln!("error {}", e),
                    }
                }
            } else {
                println!("pls type the path");
                return Ok(());
            }
        }
    }
    Ok(())
}

// eyJpdiI6IkpqblpIcmRkRU9Hbm1VUzc3cWhXM1E9PSIsInZhbHVlIjoiMktHM0hJaXk0Qkl5MjBpWWFncGhSNzgycnRCd1VrQXBBajBtajkvZHVNU2tKNENsVGtMd2hrS2F5TmJmZG1LaTNYbDg2V0xZakEwRUN0Y1ZMYStQUjVhUHNFMHZLOFV5ZXJuM1ZSRS9aa3czaUtLanIwMnBUMXVEdktxN25Vb3J3WFdVZ08rWmFDSVY5NjRySThTVDFRPT0iLCJtYWMiOiIzYjRiYTBkNmJmODFkOTNmZWFkZDIwYmFjNmNhMmE3MThiNzQ2YmEwY2I0Y2QxNzNmZWE2ODNmYTFjOTY5YTQ5IiwidGFnIjoiIn0%3D
