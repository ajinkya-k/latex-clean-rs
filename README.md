# latex-clean-rs

This command line tool has a single purpose: removing latex temporary files.
It is basically the rust port of a shell script written by Danica Sutherland (see [below](#This-program-could-have-been-a-shell-script))

> [!CAUTION]
> This project is still in development (mostly because each function isn't unit tested yet), but I do personally use it, and it has replaced the shell script I used for years.

# Usage

Simply run the following on the command line:

```bash
latex-clean <path>
```
The `path` must be to a `tex` file, a `pdf` file, or a directory.
If the `path` is to a file, the axillary files associated with that file are deleted.
If the `path` is to a directory, **all** auxiliary files in that directory are deleted.

# This program could have been a shell script

Yes, and in fact it was.
This program is basically just a `rust` port of a [shell script](https://gist.github.com/djsutherland/266983#file-latex-clean-sh) written by [Danica Sutherland](https://gist.github.com/djsutherland).
I ported this to rust for a few reasons:

- I find shell scripts hard to understand
- I wanted this to be cross-platform
- I wanted to teach myself how to publish a rust binary
