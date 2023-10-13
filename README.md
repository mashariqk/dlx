DeleteX (dlx)
=============

This cli tool can be used to delete files with a specific
extension from within a directory

Usage
-----

After cloning the repo run the following command to
generate the binary in the target/release folder

```shell
cargo build --release
```

This tool takes two mandatory params

path - the directory where the files are

ext - the extension to delete

```shell
dlx --path . --ext mp4
```

Providing the --sub flag will also traverse subdirectories

Providing the --sub flag will also traverse subdirectories

Providing the --dry flag will only print out names of matching files but not delete them

Providing the --created-before (in seconds) will only pick files older than the provided time