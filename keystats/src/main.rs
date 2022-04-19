use std::{
    collections::HashMap,
    sync::{atomic::AtomicUsize, Arc, Mutex, RwLock},
};

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use walkdir::{DirEntry, WalkDir};

use structopt::StructOpt;

fn file_filter(path: &DirEntry) -> bool {
    // println!("{:?}", path);
    let file_exts = vec!["rs", "py", "txt", "md"];
    let file_name = path.file_name().to_str().unwrap();
    let file_ext = file_name.split(".").last().unwrap();
    file_exts.contains(&file_ext)
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

fn str_to_ngram(string: &[u8], n: usize) -> HashMap<Vec<u8>, usize> {
    let mut ngrams: HashMap<Vec<u8>, usize> = HashMap::new();
    str_append_ngram(&mut ngrams, string, n);
    ngrams
}

#[derive(Debug, StructOpt)]
struct Opt {
    /// whether to provide status updates
    #[structopt(short, long)]
    updates: bool,

    /// whether to calculate two-letter word frequencies
    #[structopt(short, long)]
    doubles: bool,

    /// whether to sort by frequency before printing
    #[structopt(short, long)]
    sort: bool,

    /// the number of ngrams to use
    #[structopt(short, long, default_value = "2")]
    n: usize,

    /// Set which directory to index
    #[structopt(default_value = ".")]
    directory: String,
}

fn main() {
    let opt: Opt = Opt::from_args();

    let mut ascii_nums = vec![];
    for i in 0..opt.n {
        ascii_nums.push(HashMap::new());
    }

    let now = Arc::new(RwLock::new(std::time::SystemTime::now()));
    let now_total = std::time::SystemTime::now();
    let num_files = AtomicUsize::new(0);
    let glob = WalkDir::new(opt.directory)
        .into_iter()
        .filter_map(Result::ok)
        .filter(file_filter)
        .collect::<Vec<_>>();

    let glob_len = glob.len();
    eprintln!(
        "Indexing {} files took {:?}",
        glob_len,
        now_total.elapsed().unwrap()
    );

    let now_total = std::time::SystemTime::now();

    glob.iter().for_each(|entry| {
        num_files.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        // read file to u8 otherwise skip
        let file_content = std::fs::read(entry.path()).unwrap_or(vec![]);
        // get file_content length
        // println!("{}: {}", entry.display(), file_content.len());
        // count ascii chars
        for (i, ascii_num) in ascii_nums.iter_mut().enumerate() {
            str_append_ngram(ascii_num, &file_content, i + 1);
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
            // for chr in str {
            //     (32..127).contains(chr);
            // }
            if num > &0 && str.iter().all(|chr| (32..127).contains(chr)) {
                ascii_double_lut.push((str, num));
            }
        }
        ascii_double_lut.sort_by_key(|(_, num)| *num);
        ascii_double_luts.push(ascii_double_lut);
    }
    for (i, ascii_double_lut) in ascii_double_luts.iter_mut().enumerate() {
        ascii_double_lut.reverse();
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
        std::fs::write(format!("stats_{}.txt", i + 1), output).unwrap();
    }

    eprintln!(
        "{} files in {:?}",
        num_files.load(std::sync::atomic::Ordering::Relaxed),
        now_total.elapsed().unwrap()
    );
}
