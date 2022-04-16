use std::{
    path::PathBuf,
    sync::{atomic::AtomicUsize, Arc, Mutex, RwLock},
};

use rayon::iter::{IntoParallelRefIterator, ParallelBridge, ParallelIterator};
use walkdir::{DirEntry, WalkDir};

fn file_filter(path: &DirEntry) -> bool {
    // println!("{:?}", path);
    let file_exts = vec!["rs", "py", "txt", "md"];
    let file_name = path.file_name().to_str().unwrap();
    let file_ext = file_name.split(".").last().unwrap();
    file_exts.contains(&file_ext)
}

fn main() {
    let ascii_nums = Arc::new(Mutex::new([0; 128]));

    let mut now = Arc::new(RwLock::new(std::time::SystemTime::now()));
    let num_files = AtomicUsize::new(0);
    let glob = WalkDir::new(".")
        .into_iter()
        .filter_map(Result::ok)
        .filter(file_filter)
        .collect::<Vec<_>>();

    println!("{:?}", glob.len());

    glob.par_iter().for_each(|entry| {
        num_files.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        // read file to u8 otherwise skip
        let file_content = std::fs::read(entry.path()).unwrap_or(vec![]);
        // get file_content length
        // println!("{}: {}", entry.display(), file_content.len());
        // count ascii chars
        let mut ascii_num = [0; 128];
        for char in file_content {
            ascii_num.get_mut(char as usize).map(|c| *c += 1);
            // ascii_nums[char as usize] += 1;
        }

        ascii_nums.lock().unwrap().iter_mut().zip(&ascii_num).for_each(|(ascii, asciii)| {
            *ascii += asciii;
        });

        if now.read().unwrap().elapsed().unwrap().as_secs() > 1 {
            println!(
                "{} files processed",
                num_files.load(std::sync::atomic::Ordering::Relaxed)
            );
            *now.write().unwrap() = std::time::SystemTime::now();
        }
    });

    // print ascii chars
    for (i, num) in ascii_nums.lock().unwrap().iter().enumerate() {
        if *num > 0 {
            println!("{}: {}", i as u8 as char, num);
        }
    }

    println!("{} files in {:?}", num_files.load(std::sync::atomic::Ordering::Relaxed), now.read().unwrap().elapsed().unwrap());
}
