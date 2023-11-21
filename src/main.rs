use std::{env, fs, path::PathBuf};

use walkdir::{WalkDir, DirEntry};


fn main() {
    parse_args();

    WalkDir::new(".").follow_links(true).into_iter().
    filter_entry(|x| is_var(x)).
    filter_map(|v| v.ok()).
    filter_map(|entry| file(entry)).
    for_each(|x| {
        work(x.path().display().to_string().as_str(), x.file_name().to_str().unwrap_or(""));
    });

}

fn work(path: &str, filename: &str) {
    let sep: Vec<&str> = filename.split('.').collect();
    match sep.get(0) {
        Some(x) => {
            let author = x.trim();
            let from = PathBuf::from(path);
            let to = PathBuf::from(format!("{}/{}", author, filename).as_str());
            match fs::create_dir_all(&author) {
                Ok(_) => {},
                Err(e) => println!("创建目录失败: {} {}", e, to.display()),
            }
            match fs::rename(&from, &to) {
                Ok(_) => println!("移动文件: {} -> {}", from.display(), to.display()),
                Err(e) => println!("移动文件失败: {} -> {} {}", from.display(), to.display(), e),
            }
        },
        None => 
            println!("获取作者名称失败: {}", filename)
    }
}

fn file(entry: DirEntry) -> Option<DirEntry> {
    match entry.metadata() {
        Ok(metadata) => metadata.is_file().then(|| entry),
        Err(_) => { println!("读取路径信息错误：{}", entry.path().display()); return None; },
    }
}

fn is_var(entry: &DirEntry) -> bool {
    match entry.metadata() {
        Ok(metadata) => {
            if metadata.is_dir() {
                return true;
            }
            return entry.file_name()
            .to_str()
            .map(|s| s.ends_with(".var")).unwrap_or(false);
        },
        Err(_) => { println!("读取路径信息错误：{}", entry.path().display()); return false; },
    }
}


fn parse_args() {

    let mut _path = "./AddonPackages";

    let args: Vec<String> = env::args().collect();

    let mut param = _path;
    match args.get(2) {
        Some(value) => {
            param = value;
        },
        _ => {},
    }
    match args.get(1) {
        Some(command) => {
            if command == "-conf" {
                _path = param;
            }
        },
        _ => {},
    };

    match env::set_current_dir(_path) {
        Ok(_) => {},
        Err(e) => panic!("{}", e),
    };
}