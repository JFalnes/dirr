# Dirr - A Rust Directory Tree Printer

`dirr` is a simple command-line utility built in Rust that displays the directory tree structure, allowing the user to optionally exclude specific directories.

## Features

- Print the directory tree structure in a visual manner.
- Option to exclude specific directories from the output.

## Usage

To use `dirr`, navigate to the directory you want to display and run the compiled binary:

```bash
$ dirr
```

If you wish to exclude specific directories from the output, use the `-o` flag followed by the directory name:

```bash
$ dirr -o example
```

In this case, the directory named `example` will be excluded from the printed tree.

## How It Works

1. `dirr` starts by reading the current directory.
2. It iterates through each item (file or sub-directory) present in the directory.
3. For each item, it checks whether the item is in the exclusion list.
4. It then recursively processes each sub-directory, repeating the above steps.
5. Once all directories and files have been processed, it prints the directory tree, showing the structure and depth using a combination of `|--` and `|   ` prefixes.

## Building from Source

To build `dirr` from source, ensure you have Rust and Cargo installed. Navigate to the project's root directory and run:

```bash
$ cargo build --release
```

This will produce a binary in the `target/release` directory. You should add the compiled `.exe` file to your environment path to easily use `dirr` from any location in your terminal.

### Adding to the Environment Path

After building the project, you can add the executable to your environment path to run it from any location. Here is how you can do it:

1. Copy the `dirr.exe` file from the `target/release` directory.
2. Paste it in a directory that is in your system’s `PATH`.
3. Now you can run the `dirr` command from anywhere in your terminal/command prompt.

If you're unsure how to add a directory to your system's `PATH`, you can find many guides available online based on your operating system.

## Contribution

If you find any issues or would like to add new features, please open an issue or submit a pull request.

## License

This project is open-source. Feel free to use, modify, and distribute it as you see fit.
