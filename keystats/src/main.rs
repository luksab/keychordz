use std::path::PathBuf;

use glob::glob;

fn file_filter(path: &PathBuf) -> bool {
    let file_exts = vec!["rs", "py", "txt", "md"];
    let file_name = path.file_name().unwrap().to_str().unwrap();
    let file_ext = file_name.split(".").last().unwrap();
    file_exts.contains(&file_ext)
}

fn main() {
    let mut ascii_nums = [0; 128];

    for entry in glob("**/*")
        .unwrap()
        .filter_map(Result::ok)
        .filter(file_filter)
    {
        // read file to string
        let file_content = std::fs::read_to_string(&entry).unwrap();
        // get file_content length
        println!("{}: {}", entry.display(), file_content.len());
        // count ascii chars
        for char in file_content.bytes() {
            ascii_nums[char as usize] += 1;
        }
    }

    // print ascii chars
    for (i, num) in ascii_nums.iter().enumerate() {
        if *num > 0 {
            println!("{}: {}", i as u8 as char, num);
        }
    }
}
