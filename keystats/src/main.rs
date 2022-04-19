use std::sync::{atomic::AtomicUsize, Arc, Mutex, RwLock};

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

    /// Set which directory to index
    #[structopt(default_value = ".")]
    directory: String,
}

fn main() {
    let opt: Opt = Opt::from_args();

    let ascii_nums = Arc::new(Mutex::new([0; 128]));

    let ascii_doubles = Arc::new(Mutex::new([0; 128 * 128]));

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

    glob.par_iter().for_each(|entry| {
        num_files.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        // read file to u8 otherwise skip
        let file_content = std::fs::read(entry.path()).unwrap_or(vec![]);
        // get file_content length
        // println!("{}: {}", entry.display(), file_content.len());
        // count ascii chars
        let mut ascii_num = [0; 128];
        for char in &file_content {
            ascii_num.get_mut(*char as usize).map(|c| *c += 1);
            // ascii_nums[char as usize] += 1;
        }

        let mut ascii_double = [0; 128 * 128];

        for chars in file_content.windows(2) {
            if !(0..128).contains(&chars[0]) || !(0..128).contains(&chars[1]) {
                // make sure chars are ascii, else continue
                continue;
            }
            ascii_double[chars[0] as usize * 128 + chars[1] as usize] += 1;
        }

        // std::thread::sleep(std::time::Duration::from_millis(10));

        ascii_nums
            .lock()
            .unwrap()
            .iter_mut()
            .zip(&ascii_num)
            .for_each(|(ascii, asciii)| {
                *ascii += asciii;
            });

        ascii_doubles
            .lock()
            .unwrap()
            .iter_mut()
            .zip(&ascii_double)
            .for_each(|(ascii, asciii)| {
                *ascii += asciii;
            });

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

    if !opt.doubles {
        // print ascii chars
        let mut sum = 0;
        let mut ascii_lut = vec![];
        for (i, num) in ascii_nums.lock().unwrap().iter().enumerate() {
            if *num > 0 && (32..127).contains(&i) {
                ascii_lut.push((i as u8 as char, *num));
                sum += *num;
            }
        }
        if opt.sort {
            ascii_lut.sort_by(|a, b| b.1.cmp(&a.1));
        }

        for (char, num) in ascii_lut {
            println!("{} {}", char, num);
        }

        eprintln!("{} total ascii chars", sum);
    }

    if opt.doubles {
        let mut ascii_double_lut = vec![];
        for (i, num) in ascii_doubles.lock().unwrap().iter().enumerate() {
            let i1 = i / 128;
            let i2 = i % 128;
            if *num > 0 && (32..127).contains(&i1) && (32..127).contains(&i2) {
                ascii_double_lut.push(((i1 as u8 as char, i2 as u8 as char), *num));
            }
        }
        ascii_double_lut.sort_by_key(|(_, num)| *num);
        // ascii_double_lut.reverse();
        for (chars, num) in ascii_double_lut {
            println!("{}{}: {}", chars.0, chars.1, num);
        }
    }

    eprintln!(
        "{} files in {:?}",
        num_files.load(std::sync::atomic::Ordering::Relaxed),
        now_total.elapsed().unwrap()
    );
}
