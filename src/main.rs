use std::fs;
use regex;

fn find_files(path: &str, handler: &impl Fn(&str)) {
    let data = fs::metadata(path);
    if let Ok(data) = data {
        if data.is_file() {
            handler(path);
        } else if data.is_dir() {
            let sub_dir = fs::read_dir(path);
            if let Ok(dir) = sub_dir {
                for entry in dir {
                    if let Ok(entry) = entry {
                        find_files(entry.path().to_str().unwrap(), handler);
                    }
                }
            }
        }
    }
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    assert_eq!(args.len(), 4, "Invalid argument list missing path or str");

    let path = &args[1];
    let str_pattern = &args[2];
    let replace = &args[3];

    let pattern_regex = regex::Regex::new(str_pattern)
        .expect("invalid regex pattern");

    find_files("./", &|path: &str| {
        let res = fs::read_to_string(path);
        if let Ok(content) = res {
            let replaced_content = pattern_regex.replace_all(
                &content,
                replace
            );
            fs::write(path, replaced_content.to_string());
        }
    });
}
