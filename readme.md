# Overview

Minimal Rust CLI app with no external dependencies. Creates text files. May
create parent directories recursively, override existing files and output
verbosely.

# Description

mkfile \[OPTION\]... PATH...

Options:
- -v --verbose - print a message for each file;
- -p --parents - create parent directories recursively;
- -o --override - override already existing files;
- --help - display help message and exit;
- --version - display version message and exit.

# Usage example

Create a bunch of files in verbose mode. Some of the files couldn't be created because we didn't include -p (--parent) option to create parent directories.

```
$ mkfile -v /file.txt /test/file.txt /home/user/file.txt /home/user/test/file.txt

/file.txt: Permission denied (os error 13)
/test/file.txt: No such file or directory (os error 2)
/home/user/file.txt: Created
/home/user/test/file.txt: No such file or directory (os error 2)
```

We failed to create a file because it already exist, but we didn't include -o (--override) option to override it.

```
$ mkfile -v ~/file.txt

/home/user/file.txt: File already exist
```

# TODO

- Create binary files
- Default text parameter for all files
- Default binary parameter for all files in binary mode
- chmod parameter
