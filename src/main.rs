use std::{
    ffi::OsString,
    fs::{self, read_dir, File},
    io::{Read, Write},
    path::PathBuf,
};

use clap::Parser;
use indicatif::ProgressBar;
use zip::{write::FileOptions, ZipWriter};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// 文件路径
    #[arg(short, long)]
    path: Option<String>,
    /// 文件拓展名
    #[arg(short, long)]
    exp: Option<String>,
}

fn get_dirs(path: String) -> Vec<(PathBuf, OsString)> {
    fs::read_dir(path)
        .unwrap()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_dir())
        .map(|e| (e.path(), e.file_name()))
        .collect::<Vec<_>>()
}

fn create_new_zip(name: OsString, path: PathBuf) -> ZipWriter<File> {
    let mut full_path = path.as_os_str().to_os_string();
    full_path.push(name);
    ZipWriter::new(File::create(full_path).unwrap())
}

fn init_process_bar(target_path: PathBuf, file_name: OsString) -> ProgressBar {
    let files_num = read_dir(target_path).unwrap().count();
    let process_bar = ProgressBar::new(files_num as u64);
    process_bar.set_message("正在压缩：".to_string() + file_name.to_str().unwrap());
    return process_bar;
}
fn get_files(path: PathBuf) -> Vec<(OsString, PathBuf)> {
    read_dir(path)
        .unwrap()
        .filter_map(|e| e.ok())
        .map(|e| (e.file_name(), e.path()))
        .collect::<Vec<_>>()
}

fn write_file(
    target: &mut ZipWriter<File>,
    name: OsString,
    path: PathBuf,
    zip_option: FileOptions,
) {
    let mut buf = Vec::<u8>::new();
    target
        .start_file(name.to_string_lossy(), zip_option)
        .unwrap();
    let mut f = File::open(path).unwrap();
    f.read_to_end(&mut buf).unwrap();
    target.write_all(&buf).unwrap();
    buf.clear();
}

fn main() {
    let args = Args::parse();
    let path = args.path.unwrap_or(".".to_owned());
    let exp = args.exp.unwrap_or(".cbz".to_string());

    let options = FileOptions::default().compression_method(zip::CompressionMethod::Stored);
    let dirs = get_dirs(path);
    for dir in dirs {
        let mut file_name = dir.1;
        file_name.push(exp.clone());
        let mut zip_file = create_new_zip(file_name.clone(), dir.0.clone());

        let process_bar = init_process_bar(dir.0.clone(), file_name.clone());
        let files = get_files(dir.0.clone());
        for file in files {
            write_file(&mut zip_file, file.0, file.1, options);
            process_bar.inc(1);
        }
        zip_file.finish().unwrap();
    }
}
