use std::sync::{atomic::AtomicUsize, Arc, Mutex, RwLock};

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
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

    let ascii_doubles = Arc::new(Mutex::new([0; 128 * 128]));

    let now = Arc::new(RwLock::new(std::time::SystemTime::now()));
    let now_total = std::time::SystemTime::now();
    let num_files = AtomicUsize::new(0);
    let glob = WalkDir::new(".")
        .into_iter()
        .filter_map(Result::ok)
        .filter(file_filter)
        .collect::<Vec<_>>();

    let glob_len = glob.len();
    println!("{} files in total", glob.len());

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

        let mut ascii_doubles = [0; 128 * 128];

        for chars in file_content.windows(2) {
            if !(0..128).contains(&chars[0]) || !(0..128).contains(&chars[1]) {
                // make sure chars are ascii, else continue
                continue;
            }
            ascii_doubles[chars[0] as usize * 128 + chars[1] as usize] += 1;
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

        if now.read().unwrap().elapsed().unwrap().as_secs() > 1 {
            let num_processed = num_files.load(std::sync::atomic::Ordering::Relaxed);
            let percent_done = num_processed as f32 / glob_len as f32;
            let time = now_total.elapsed().unwrap().as_secs_f32();
            let eta = (time / percent_done) - time;
            println!(
                "{}/{} files processed. {}% done. ETA: {:?}",
                num_files.load(std::sync::atomic::Ordering::Relaxed),
                glob_len,
                percent_done * 100.,
                std::time::Duration::from_secs_f32(eta)
            );
            *now.write().unwrap() = std::time::SystemTime::now();
        }
    });

    // print ascii chars
    let mut sum = 0;
    let mut ascii_lut = vec![];
    for (i, num) in ascii_nums.lock().unwrap().iter().enumerate() {
        if *num > 0 && (32..127).contains(&i) {
            println!("{}: {}", i as u8 as char, num);
            ascii_lut.push((i as u8 as char, *num));
            sum += *num;
        }
    }
    println!("total chars: {}", sum);

    println!("{:?}", ascii_lut);
    // sort by num
    ascii_lut.sort_by(|a, b| b.1.cmp(&a.1));
    println!("{:?}", ascii_lut);

    println!(
        "{} files in {:?}",
        num_files.load(std::sync::atomic::Ordering::Relaxed),
        now.read().unwrap().elapsed().unwrap()
    );
}
