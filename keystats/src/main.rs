use std::{
    collections::HashMap,
    sync::{atomic::AtomicUsize, Arc, RwLock},
};

use colored::Colorize;
use rayon::prelude::*;
use walkdir::{DirEntry, WalkDir};

use structopt::StructOpt;

fn filter_by_extension(extensions: &[&str], entry: &DirEntry) -> bool {
    extensions.iter().any(|ext| {
        entry
            .path()
            .extension()
            .map_or(false, |e| e.to_str() == Some(ext))
    })
}

fn filter_by_not_target(entry: &DirEntry) -> bool {
    !entry
        .path()
        .as_os_str()
        .to_str()
        .map_or(false, |s| s.contains("/target/"))
}

fn file_filter(path: &DirEntry) -> bool {
    let file_exts = vec![
        "rs", "toml", "wgsl", "py", "js", "md", "c", "cpp", "h", "hpp", "html", "css", "sh",
        "java", "go", "ts", "yml", "yaml", "json",
    ];
    filter_by_extension(&file_exts, path) && filter_by_not_target(path)
}

fn str_append_ngram(ngrams: &mut HashMap<Vec<u8>, usize>, s: &[u8], n: usize) {
    for chars in s.windows(n) {
        let mut all_ascii = true;
        for chr in chars {
            all_ascii &= (0..128).contains(chr);
        }
        if !all_ascii {
            // make sure chars are ascii, else continue
            continue;
        }
        // add 1 to element in hashmap
        match ngrams.get_mut(chars) {
            Some(count) => *count += 1,
            None => {
                let _ = ngrams.insert(chars.to_vec(), 1);
            }
        }
    }
}

#[derive(Debug, StructOpt)]
struct Opt {
    /// Enable periodic status updates
    #[structopt(short, long)]
    updates: bool,

    /// Disable sorting by frequency before printing
    #[structopt(short = "s", long)]
    disable_sort: bool,

    /// Print files to process
    #[structopt(short, long)]
    print_files: bool,

    /// The number of ngrams to use
    #[structopt(short, long, default_value = "1")]
    ngrams: usize,

    /// Calculate all n from 1 to ngrams
    #[structopt(short, long)]
    multi: bool,

    /// Output to files beginning with
    #[structopt(short, long)]
    output: Option<String>,

    /// Print stats
    #[structopt(short = "t", long)]
    print_time: bool,

    /// Set which directory to index
    #[structopt(default_value = ".")]
    directory: Vec<String>,
}

fn main() {
    let opt: Opt = Opt::from_args();

    let mut ascii_nums = vec![];
    if opt.multi {
        for _ in 0..opt.ngrams {
            ascii_nums.push(HashMap::new());
        }
    } else {
        ascii_nums.push(HashMap::new());
    }

    let now = Arc::new(RwLock::new(std::time::SystemTime::now()));
    let now_total = std::time::SystemTime::now();
    let num_files = AtomicUsize::new(0);
    let glob = opt
        .directory
        .iter()
        .flat_map(|dir| {
            if !std::path::Path::new(dir).exists() {
                let out = format!("Path \"{}\" does not exist!", dir);
                eprintln!("{}", out.red());
            }
            WalkDir::new(dir)
                .into_iter()
                .filter_map(Result::ok)
                .filter(file_filter)
        })
        .collect::<Vec<_>>();

    if opt.print_files {
        eprintln!("processing files:");
        for file in &glob {
            eprintln!("{}", file.path().display());
        }
    }

    let glob_len = glob.len();
    if opt.print_time {
        eprintln!(
            "Indexing {} files took {:?}",
            glob_len,
            now_total.elapsed().unwrap()
        );
    }

    let now_total = std::time::SystemTime::now();

    glob.iter().for_each(|entry| {
        num_files.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        // read file to u8 otherwise skip
        let file_content = std::fs::read(entry.path()).unwrap_or(vec![]);
        // count ascii chars
        if opt.multi {
            ascii_nums
                .par_iter_mut()
                .enumerate()
                .for_each(|(i, ascii_num)| {
                    str_append_ngram(ascii_num, &file_content, i + 1);
                });
        } else {
            str_append_ngram(&mut ascii_nums[0], &file_content, opt.ngrams);
        }

        if now.read().unwrap().elapsed().unwrap().as_secs() > 1 && opt.updates {
            let num_processed = num_files.load(std::sync::atomic::Ordering::Relaxed);
            let percent_done = num_processed as f32 / glob_len as f32;
            let time = now_total.elapsed().unwrap().as_secs_f32();
            let eta = (time / percent_done) - time;
            eprintln!(
                "{}/{} files processed. {}% done. ETA: {:?}",
                num_files.load(std::sync::atomic::Ordering::Relaxed),
                glob_len,
                percent_done * 100.,
                std::time::Duration::from_secs_f32(eta)
            );
            *now.write().unwrap() = std::time::SystemTime::now();
        }
    });

    let mut ascii_double_luts = vec![];
    for ascii_num in ascii_nums.iter() {
        let mut ascii_double_lut = vec![];
        for (str, num) in ascii_num.into_iter() {
            if num > &0 && str.iter().all(|chr| (32..127).contains(chr)) {
                ascii_double_lut.push((str, num));
            }
        }
        if !opt.disable_sort {
            ascii_double_lut.sort_by(|a, b| b.1.cmp(&a.1));
        }
        ascii_double_luts.push(ascii_double_lut);
    }
    for (i, ascii_double_lut) in ascii_double_luts.iter_mut().enumerate() {
        let mut output = String::new();
        for (chars, num) in ascii_double_lut {
            // println!(
            //     "{}: {}",
            //     chars.iter().map(|chr| *chr as char).collect::<String>(),
            //     num
            // );
            output.push_str(&format!(
                "{}: {}\n",
                chars.iter().map(|chr| *chr as char).collect::<String>(),
                num
            ));
        }
        match opt.output {
            Some(ref output_file) => {
                std::fs::write(format!("{}_{}.txt", output_file, i + 1), output).unwrap();
            }
            None => {
                if opt.multi {
                    println!("{}gram", i + 1);
                }
                print!("{}", output);
            }
        }
    }

    if opt.print_time {
        eprintln!(
            "Processed {} files in {:?}",
            num_files.load(std::sync::atomic::Ordering::Relaxed),
            now_total.elapsed().unwrap()
        );
    }
}
