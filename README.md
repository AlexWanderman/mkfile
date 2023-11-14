# Overview ![Crates.io](https://img.shields.io/crates/v/mkfile)

Minimal Rust CLI app with no external dependencies. Creates text files. May
create parent directories recursively, override existing files and output
verbosely. Default text for new files supported.

Install with `cargo install mkfile`.

# Description

mkfile \[OPTION\]... PATH...

Options:
- -d --dry - perform "dry" run, always verbose;
- -v --verbose - print a message for each file;
- -p --parents - create parent directories recursively;
- -o --override - override already existing files;
- -T --text "STRING" - default text for each file;
- --help - display help message and exit;
- --version - display version message and exit.

# Usage example

Basic example. Create new file silently.
```
$ mkfile file.txt
```

Create multiple files (with text, verbosely). 
```
$ mkfile file1.txt file2.txt file3.txt -vT "Default text"
/home/user/file1.txt: Created
/home/user/file2.txt: Created
/home/user/file3.txt: Created
```

Create file with parent directory (verbosely).
```
$ mkfile -vp parent/file.txt
/home/user/parent/file.txt: Created with parent
```

Dry run example. Be aware that /root_file.txt will not be created without root privileges.
```
$ mkfile -d new_dir/file.txt new_file.txt existing_file.txt /root_file.txt
/home/user/new_dir/file.txt: Parent does not exist
/home/user/new_file.txt: To be created 
/home/user/existing_file.txt: Already exist
/root_file.txt: To be created
```

# TODO

- Create tests
- chmod parameters
