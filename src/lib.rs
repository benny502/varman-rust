use std::{error::Error, env, path::PathBuf, fs};

use walkdir::{WalkDir, DirEntry};

pub struct Config {
    pub path: String
}

impl Config {
    pub fn new() -> Result<Config, Box<dyn Error>>{

        let mut args = env::args();

        args.next();

        let command = args.next().unwrap_or("".to_string());

        let path;

        if command.is_empty() {

            path = "./AddonPackages".to_string();

        }else {

            if command == "-conf" {
                path = args.next().unwrap_or("".to_string());
    
                if path.is_empty() {
                    return Err("请输入AddonPackages路径".into());
                }
            }else {
                return Err(format!("{} 没有这个指令 eg. -conf ./AddonPackages", command).into());
            }

        }

        Ok(Config { path: path })

    }
}


pub fn run() {

    WalkDir::new(".").follow_links(true).into_iter().
    filter_entry(|x| is_var(x)).
    filter_map(|v| v.ok()).
    filter_map(|entry| file(entry)).
    for_each(|x| {
        if let Err(e) = work(x.path().as_os_str().to_str().unwrap_or(""), x.file_name().to_str().unwrap_or("")) {
            eprintln!("{}", e);
        }
    });

}

fn work(path: &str, filename: &str) ->Result<(), Box<dyn Error>> {

    let author = filename.split('.').next().unwrap_or("");

    if author.is_empty() {
        return Err(format!("获取作者名称失败: {}", filename).into());
    }

    let from = PathBuf::from(path);
    let to = PathBuf::from(format!("{}/{}", author, filename).as_str());

    if let Err(e) = fs::create_dir_all(&author) {
        return Err(format!("创建目录失败: {} {}", to.display(), e).into());
    }

    match fs::rename(&from, &to) {
        Ok(_) => println!("移动文件: {} -> {}", from.display(), to.display()),
        Err(e) => {
            return Err(format!("移动文件失败: {} -> {} {}", from.display(), to.display(), e).into());
        },
    };

    Ok(())
}

fn file(entry: DirEntry) -> Option<DirEntry> {
    match entry.metadata() {
        Ok(metadata) => metadata.is_file().then(|| entry),
        Err(_) => { eprintln!("读取路径信息错误：{}", entry.path().display()); None },
    }
}

fn is_var(entry: &DirEntry) -> bool {
    match entry.metadata() {
        Ok(metadata) => {

            if metadata.is_dir() {
                return true;
            }

            entry.file_name()
            .to_str()
            .map(|s| s.ends_with(".var")).unwrap_or(false)
        },
        Err(_) => { eprintln!("读取路径信息错误：{}", entry.path().display()); false },
    }
}
