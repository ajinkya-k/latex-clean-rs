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

fn is_tex_aux<T: AsRef<Path>>(path: &T) -> Result<bool, std::io::Error> {
    // println!("checking path {}", path.as_ref().display());
    let pathref = path.as_ref();
    let ext = match pathref.extension() {
        Some(x) => x,
        None => return Ok(false),
    }
    .to_str()
    .ok_or_else(|| {
        Error::new(
            std::io::ErrorKind::InvalidData,
            format!("BUG: Problem OsStr to str for path {}", pathref.display()),
        )
    })?;
    // .expect(&format!(
    //     "BUG: Problem OsStr to str for path {}",
    //     pathref.display()
    // ));

    let fl = EXTS.contains(&ext)
        || pathref
            .to_str()
            .ok_or_else(|| {
                Error::new(
                    std::io::ErrorKind::InvalidData,
                    format!("BUG: Problem OsStr to str for path {:}", pathref.display()),
                )
            })?
            .contains(r#"synctex.gz"#); //TODO: remove unwrap
    Ok(fl)
}

fn process_dir<T: AsRef<Path>>(fld: T) -> Result<Vec<PathBuf>, std::io::Error> {
    println!("Processing directory {}", fld.as_ref().display());
    let files = read_dir(&fld)?
        .filter_map(|res| res.ok())
        .map(|entry| entry.path())
        .filter(|path| {
            path.is_file()
                && is_tex_aux(&path).unwrap_or_else(|e| {
                    eprintln!("{}, Skipping the file {}", e, path.display());
                    false
                })
        }) // https://users.rust-lang.org/t/collections-filter-and-error-handling/32871/2
        .collect::<Vec<PathBuf>>();
    // println!("{:?}", files);
    Ok(files)
}

fn process_file<T: AsRef<Path>>(path: T) -> Result<Vec<PathBuf>, std::io::Error> {
    let fld = path.as_ref();
    let pardir = match fld.extension() {
        Some(x) => match x.to_str() {
            Some("tex") | Some("pdf") => fld.parent().ok_or_else(|| {
                Error::new(
                    std::io::ErrorKind::InvalidData,
                    format!("Cannot get parent directory for {}", fld.display()),
                )
            }),
            _ => Err(Error::new(
                std::io::ErrorKind::InvalidInput,
                "input must a directory, tex, or pdf file",
            )),
        },
        None => Err(Error::new(
            std::io::ErrorKind::InvalidInput,
            "input must a directory, tex, or pdf file",
        )),
    }?;

    let pardir = match pardir.as_os_str().is_empty() {
        true => Path::new("."),
        false => pardir,
    };
    // println!("Parent directory is {}", pardir.display());
    let fname = fld
        .file_stem()
        .ok_or_else(|| {
            Error::new(
                std::io::ErrorKind::InvalidFilename,
                format!(
                    "something wrong with file stem for the filename {}",
                    fld.display(),
                ),
            )
        })?
        .to_str()
        .ok_or_else(|| {
            Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Something went wrong when priocessing {}", fld.display()),
            )
        })?;
    // println!("{:?}", fname);
    let files = read_dir(pardir)?
        .filter_map(|res| res.ok())
        .map(|entry| entry.path())
        .filter(|path| match path.file_name() {
            Some(x) => match x.to_str() {
                Some(x) => {
                    x.starts_with(fname)
                        && is_tex_aux(&path).unwrap_or_else(|e| {
                            eprintln!("{}, Skipping the file {}", e, path.display());
                            false
                        })
                }
                None => {
                    eprintln!(
                        "Something off when proessing file {}, skipping",
                        path.display()
                    );
                    false
                }
            },
            None => {
                eprintln!("couldn't process file {}, skipping", path.display());
                false
            }
        })
        .collect::<Vec<_>>();
    Ok(files)
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
        true => process_dir(fld),
        false => process_file(fld),
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
