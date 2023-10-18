use std::collections::VecDeque;
use std::env;
use std::fs;
use std::fs::File;
use std::path::Path;

fn main() {
    // Collect args and remove the executable path
    let mut args: VecDeque<String> = env::args().collect();
    args.pop_front();

    // Return if no args
    if args.is_empty() {
        return;
    }

    // Options and paths
    let mut options = Vec::<String>::new();
    let mut options_wrong = Vec::<String>::new();
    let mut paths = Vec::<&Path>::new();

    let mut is_verbose = false;
    let mut create_parents = false;
    let mut do_override = false;

    for arg in &args {
        let a = arg.chars().nth(0);
        let b = arg.chars().nth(1);
        let c = arg.chars().nth(3);

        // Long option --option
        if a.eq(&Some('-')) && b.eq(&Some('-')) && c.is_some() {
            options.push(arg[2..].to_string());
        }
        // Short option -o (single) or -vao (multiple)
        else if a.eq(&Some('-')) && b.ne(&Some('-')) {
            for opt in arg.chars() {
                match opt {
                    '-' => {}
                    _ => options.push(opt.to_string()),
                }
            }
        }
        // Paths
        else {
            paths.push(Path::new(arg));
        }
    }

    // Proceed options
    for opt in &options {
        match opt.as_str() {
            "v" | "verbose" => is_verbose = true,
            "p" | "parents" => create_parents = true,
            "o" | "override" => do_override = true,
            _ => options_wrong.push(opt.to_string()),
        }
    }

    // Output wrong options and return
    if !options_wrong.is_empty() {
        let opt_list = options_wrong.join(", ");

        if options_wrong.len() == 1 {
            eprintln!("Wrong option: {opt_list}");
        } else {
            eprintln!("Wrong options: {opt_list}");
        }

        return;
    }

    for path in paths {
        // Skip file if exist, but do_override is false
        if path.exists() && !do_override {
            println!("{}: File already exist", path.display());
            continue;
        }

        // Create parent directories
        if create_parents {
            let parent = path.parent().unwrap_or(Path::new(""));
            if !parent.exists() {
                match fs::create_dir_all(parent) {
                    Err(e) if is_verbose => {
                        println!("{}: {}", path.display(), e);
                        continue;
                    }
                    _ => {}
                };
            }
        }

        // Create files
        match File::create(path) {
            Ok(_) if is_verbose => println!("{}: Created", path.display()),
            Err(e) if is_verbose => println!("{}: {}", path.display(), e),
            _ => {}
        };
    }
}
