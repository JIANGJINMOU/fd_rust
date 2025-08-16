use ansi_term::Color;
use clap::Parser;
use ignore::WalkBuilder;
use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};
use winapi::um::fileapi::GetFileAttributesW;
use winapi::um::winnt::{FILE_ATTRIBUTE_HIDDEN, FILE_ATTRIBUTE_SYSTEM};

#[derive(Parser, Debug)]
#[command(author, version, about = "A fast command-line search tool for Windows")]
struct Args {
    pattern: String,
    #[arg(short, long, default_value = ".")]
    directory: String,
    #[arg(short = 'C', long)]
    content: bool,
    #[arg(short, long)]
    case_insensitive: bool,
    #[arg(short, long)]
    no_recurse: bool,
    #[arg(short= 'x', long, default_value_t = 0)]
    context: usize,
    #[arg(short, long)]
    ext: Option<String>,
    #[arg(short, long)]
    include_hidden: bool,
}

fn is_hidden_or_system(path: &Path) -> bool {
    let wide = match path.to_str() {
        Some(s) => s.encode_utf16().chain(std::iter::once(0)).collect::<Vec<_>>(),
        None => return false,
    };
    let attrs = unsafe { GetFileAttributesW(wide.as_ptr()) };
    attrs != u32::MAX && (attrs & (FILE_ATTRIBUTE_HIDDEN | FILE_ATTRIBUTE_SYSTEM)) != 0
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    let search_dir = Path::new(&args.directory);

    if !search_dir.is_dir() {
        eprintln!("{}: 目录无效", Color::Red.paint(&args.directory));
        std::process::exit(1);
    }

    let regex = regex::RegexBuilder::new(&args.pattern)
        .case_insensitive(args.case_insensitive)
        .build()
        .unwrap_or_else(|e| {
            eprintln!("{}: 无效正则表达式: {}", Color::Red.paint("错误"), e);
            std::process::exit(1);
        });

    let exts = args.ext.as_ref().map(|s| s.split(',').collect::<Vec<_>>());

    let mut walk = WalkBuilder::new(search_dir);
    if args.no_recurse {
        walk.max_depth(Some(1));
    }
    walk.hidden(false).git_ignore(true);

    let mut found = 0;

    for entry in walk.build().flatten() {
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        if !args.include_hidden && is_hidden_or_system(path) {
            continue;
        }
        if let Some(exts) = &exts {
            if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                if !exts.contains(&ext) {
                    continue;
                }
            }
        }

        if !args.content {
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                if regex.is_match(name) {
                    println!("{}", Color::Green.paint(path.display().to_string()));
                    found += 1;
                }
            }
        } else {
            found += search_file_content(&path.to_path_buf(), &regex, args.context).unwrap_or(0);
        }
    }

    println!("\n{}: 找到 {} 个匹配项", Color::Cyan.paint("结果"), found);
    Ok(())
}

fn search_file_content(path: &PathBuf, regex: &Regex, ctx: usize) -> io::Result<usize> {
    let lines = io::BufReader::new(File::open(path)?)
        .lines()
        .collect::<Result<Vec<_>, _>>()?;
    let mut matches = 0;
    let mut shown = false;

    for (i, line) in lines.iter().enumerate() {
        if regex.is_match(line) {
            if !shown {
                println!("\n{}", Color::Green.paint(path.display().to_string()));
                shown = true;
            }
            let start = i.saturating_sub(ctx);
            let end = (i + ctx).min(lines.len() - 1);
            for j in start..=end {
                let prefix = if j == i {
                    Color::Red.paint(format!("{}:", j + 1))
                } else {
                    Color::White.paint(format!("{}:", j + 1))
                };
                let hl = regex.replace_all(&lines[j], |c: &regex::Captures| {
                    Color::Yellow.paint(&c[0]).to_string()
                });
                println!("{} {}", prefix, hl);
            }
            matches += 1;
        }
    }
    Ok(matches)
}