use std::{
    fs::{self, read_dir},
    path::{Path, PathBuf},
};

// Extensions to look out for
// synctex.gz is handled separte
const EXTS: [&str; 18] = [
    "aux",
    "bbl",
    "blg",
    "brf",
    "idx",
    "ilg",
    "ind",
    "lof",
    "log",
    "lol",
    "lot",
    "nav",
    "out",
    "snm",
    "tdo",
    "toc",
    "fdb_latexmk",
    "fls",
];

fn is_tex_aux<T: AsRef<Path>>(path: &T) -> bool {
    // println!("checking path {}", path.as_ref().display());
    let ext = match path.as_ref().extension() {
        Some(x) => x,
        None => return false,
    }
    .to_str()
    .expect("BUG: Problem OsStr to str for paht {path.as_ref().display()}");

    EXTS.contains(&ext)
        || path
            .as_ref()
            .to_str()
            .expect(&format!(
                "BUG: Problem OsStr to str for path {:}",
                path.as_ref().display()
            ))
            .ends_with(r#"synctex.gz"#) //TODO: remove unwrap
}

fn process_dir<T: AsRef<Path>>(fld: T) -> Vec<PathBuf> {
    println!("Processing directory {}", fld.as_ref().display());
    let files = read_dir(fld)
        .expect("error reading directory")
        .filter_map(|res| res.ok())
        .map(|entry| entry.path())
        .filter(|path| path.is_file() && is_tex_aux(&path))
        .collect::<Vec<_>>();
    // println!("{:?}", files);
    files
}

fn process_file<T: AsRef<Path>>(path: T) -> Vec<PathBuf> {
    let fld = path.as_ref();
    let pardir = match fld.extension() {
        Some(x) => match x.to_str() {
            Some("tex") | Some("pdf") => fld
                .parent()
                .expect("Cannot get parent directory for {fld.display()}"),
            _ => {
                panic!("input must a directory, tex, or pdf file");
            }
        },
        _ => {
            panic!("input must a directory, tex, or pdf file");
        }
    };

    let pardir = match pardir.as_os_str().is_empty() {
        true => Path::new("."),
        false => pardir,
    };
    // println!("Parent directory is {}", pardir.display());
    let fname = fld
        .file_stem()
        .expect("something wrong with file stem")
        .to_str()
        .expect("couldnt convert to str");
    // println!("{:?}", fname);
    let files = read_dir(pardir)
        .expect("error reading directory, {pardir:?}")
        .filter_map(|res| res.ok())
        .map(|entry| entry.path())
        .filter(|path| {
            path.file_name()
                .expect("something wrng with filename")
                .to_str()
                .expect("msg")
                .starts_with(fname)
                && is_tex_aux(&path)
        })
        .collect::<Vec<_>>();
    // println!("{:?}", files);
    files
}

fn delete_files(tempfiles: Vec<PathBuf>) {
    if tempfiles.is_empty() {
        println!("Nothing to do");
        return;
    }
    for fl in tempfiles {
        println!("Removing File: {}", fl.display());
        fs::remove_file(fl).expect("someting went wrong when deleting {fl:?}")
    }
}
fn main() {
    let fld = PathBuf::from(std::env::args().nth(1).unwrap_or(".".into()));

    let _ = fld
        .try_exists()
        .expect("Couldn't verify if this exists")
        .then(|| 1)
        .ok_or("Doesnt exist");

    let isdir = match fld.try_exists() {
        Ok(true) => fld.is_dir(),
        Ok(false) => panic!("No such file or directory"),
        Err(_) => panic!("Couldnt verify if file exists"),
    };
    let tempfiles = match isdir {
        true => process_dir(fld),
        false => process_file(fld),
    };

    delete_files(tempfiles);
}
