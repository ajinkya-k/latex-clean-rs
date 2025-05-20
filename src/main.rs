use std::{
    fs::{self, read_dir},
    io::Error,
    path::{Path, PathBuf},
    process,
};

// Extensions to look out for
// synctex.gz is handled separte
const EXTS: [&str; 20] = [
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
    "synctex",
    "synctex(busy)",
];

fn is_tex_aux<T: AsRef<Path>>(path: &T) -> bool {
    // println!("checking path {}", path.as_ref().display());
    let pathref = path.as_ref();
    let ext = match pathref.extension() {
        Some(x) => x,
        None => return false,
    }
    .to_str()
    .expect(&format!(
        "BUG: Problem OsStr to str for path {}",
        pathref.display()
    ));

    EXTS.contains(&ext)
        || pathref
            .to_str()
            .expect(&format!(
                "BUG: Problem OsStr to str for path {:}",
                pathref.display()
            ))
            .contains(r#"synctex.gz"#) //TODO: remove unwrap
}

fn process_dir<T: AsRef<Path>>(fld: T) -> Vec<PathBuf> {
    println!("Processing directory {}", fld.as_ref().display());
    let files = read_dir(&fld)
        .expect(&format!(
            "error reading directory for {}",
            fld.as_ref().display()
        ))
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
            Some("tex") | Some("pdf") => fld.parent().expect(&format!(
                "Cannot get parent directory for {}",
                fld.display()
            )),
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

fn process_path(fld: PathBuf) -> Result<Vec<PathBuf>, std::io::Error> {
    let isdir = match fld.try_exists() {
        Ok(true) => Ok(fld.is_dir()),
        Ok(false) => Err(Error::new(
            std::io::ErrorKind::NotFound,
            format!("No file or directory with path {}", &fld.display()),
        )),
        Err(e) => Err(e),
    };
    match isdir? {
        true => Ok(process_dir(fld)),
        false => Ok(process_file(fld)),
    }
}

fn delete_files(tempfiles: Vec<PathBuf>) -> Result<(), std::io::Error> {
    if tempfiles.is_empty() {
        println!("Nothing to do");
        return Ok(());
    }
    for fl in tempfiles {
        println!("Removing File: {}", fl.display());
        fs::remove_file(&fl)?
    }

    Ok(())
}

fn main() {
    let fld = PathBuf::from(std::env::args().nth(1).unwrap_or(".".into()));

    let tempfiles = match process_path(fld) {
        Ok(pathvec) => pathvec,
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1)
        }
    };

    match delete_files(tempfiles) {
        Ok(()) => (),
        Err(e) => {
            eprintln!("Problem when deleting files {}", e);
            process::exit(1)
        }
    }
}
