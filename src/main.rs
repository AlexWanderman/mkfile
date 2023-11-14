use std::{env, fs, path::Path};

fn main() {
    // Stage 0: args
    let args: Vec<String> = env::args().collect();

    // Stage 1: options
    let mut do_dry_run = false;
    let mut do_verbose_output = false;
    let mut do_create_parents = false;
    let mut do_overwrite = false;
    let mut default_text = String::new();

    let mut wrong_options = Vec::<String>::new();
    let mut paths = Vec::<String>::new();

    // Using an iterator to peek the next value inside a loop. Skipping
    // the path of the executable (with .next()).
    let mut args_iter = args.iter();
    args_iter.next();

    while let Some(arg) = args_iter.next() {
        let options = if arg.chars().all(|x| x.eq(&'-')) {
            // - or -- are wrong options
            wrong_options.push(arg.to_string());
            continue;
        } else if arg.starts_with("--") {
            // Single long option
            vec![arg.strip_prefix("--").unwrap().to_string()]
        } else if arg.starts_with("-") {
            // Single or multiple short options
            arg.strip_prefix("-")
                .unwrap()
                .chars()
                .map(|x| x.to_string())
                .collect()
        } else {
            // Not an option
            paths.push(arg.to_string());
            continue;
        };

        for option in options {
            match option.as_str() {
                "d" | "dry" => do_dry_run = true,
                "v" | "verbose" => do_verbose_output = true,
                "p" | "parents" => do_create_parents = true,
                "o" | "overwrite" => do_overwrite = true,
                "T" | "text" => {
                    // Default text was already written
                    if !default_text.is_empty() {
                        eprintln!("Error! Multiple default text options are not supported.");
                        return;
                    }

                    let text = args_iter.next();

                    // Default text was not provided
                    if text.is_none() {
                        eprintln!("Error! Default text was not provided.");
                        return;
                    }

                    default_text = text.unwrap().to_string() + "\n";
                }
                "help" => {
                    print_help_msg();
                    return;
                }
                "version" => {
                    print_version_msg();
                    return;
                }
                _ => wrong_options.push(option),
            }
        }
    }

    if !wrong_options.is_empty() {
        if wrong_options.len() == 1 {
            let option = wrong_options.first().unwrap();
            eprintln!("Error! Wrong option: {option}.");
        } else {
            let options = wrong_options.join(", ");
            eprintln!("Error! Wrong options: {options}.");
        }
        return;
    }

    // Stage 2: files
    if paths.is_empty() {
        println!("Warning! No files were provided. To get a hint use --help.\n");
        return;
    }

    for path in paths {
        let full_path_str = if path.starts_with("/") {
            path.to_string()
        } else {
            let mut full_path = env::current_dir().unwrap();
            full_path.push(&path);
            full_path.display().to_string()
        };

        let full_path = Path::new(&full_path_str);
        let full_parent_path = full_path.parent().unwrap_or(Path::new("/"));

        let mut is_overwritten = false;
        let mut is_parented = false;

        // Overwritten file
        match full_path.try_exists() {
            Ok(true) => {
                if !do_overwrite {
                    eprintln!("{}: Already exist", &full_path_str);
                    continue;
                }
                is_overwritten = true;
            }
            Err(e) if do_verbose_output => eprintln!("{}: {}", &full_path_str, e),
            _ => {}
        }

        // New file with parent
        match full_parent_path.try_exists() {
            Ok(false) => {
                if !do_create_parents {
                    eprintln!("{}: Parent does not exist", &full_path_str);
                    continue;
                }

                if !do_dry_run {
                    match fs::create_dir_all(&full_parent_path) {
                        Err(e) if do_verbose_output => eprintln!("{}: {}", &full_path_str, e),
                        _ => {}
                    }
                }

                is_parented = true;
            }
            Err(e) if do_verbose_output => eprintln!("{}: {}", &full_path_str, e),
            _ => {}
        }

        // New file
        if !do_dry_run {
            match fs::write(&full_path, &default_text) {
                Ok(()) if do_verbose_output => {
                    let created = if is_overwritten {
                        "Overwritten"
                    } else {
                        "Created"
                    };
                    let parented = if is_parented { "with parent" } else { "" };
                    println!("{}: {} {}", &full_path_str, created, parented);
                }
                Err(e) if do_verbose_output => eprintln!("{}: {}", &full_path_str, e),
                _ => {}
            }
        } else {
            let created = if is_overwritten {
                "overwritten"
            } else {
                "created"
            };
            let parented = if is_parented { "with parent" } else { "" };
            println!("{}: To be {} {}", &full_path_str, created, parented);
        }
    }
}

fn print_help_msg() {
    println!("Usage: mkfile [OPTION]... PATH...");
    println!("Create file(s), if they do not already exist.\n");
    println!("Options:");
    println!("-d --dry            perform \"dry\" run, always verbose");
    println!("-v --verbose        print a message for each file");
    println!("-p --parents        create parent directories recursively");
    println!("-o --overwrite      overwrite already existing files");
    println!("-T --text \"STRING\"  default text for every file");
    println!("   --help           display this help and exit");
    println!("   --version        output version information and exit");
}

fn print_version_msg() {
    println!("mkfile v{}", env!("CARGO_PKG_VERSION"));
}
