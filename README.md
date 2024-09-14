# copyrepo

A Rust command-line tool to fetch and save the contents of a GitHub folder, specifically designed for ingestion into Large Language Models (LLMs).

## Description

This tool allows you to easily download the contents of a specific folder from a GitHub repository. It clones the repository, extracts the contents of the specified folder, and saves them to a local file in a designated output directory. The output is formatted to include file names and paths, making it ideal for ingestion into LLMs.

## Key Features

- Fetches content from a specified GitHub folder
- Includes file names and paths in the output
- Formats content for easy ingestion into LLMs
- Customizable output directory
- Adjustable timeout for git operations

## Installation

You can install `copyrepo` using Cargo, the Rust package manager. If you don't have Rust installed, you can get it from [https://www.rust-lang.org/](https://www.rust-lang.org/).

To install `copyrepo`, run the following command:

```
cargo install copyrepo
```

This will download and install the latest version of `copyrepo` from crates.io.

## Usage

Basic usage:

```
copyrepo <GITHUB_FOLDER_URL>
```

Example:

```
copyrepo https://github.com/penumbra-zone/web/tree/main/packages/ui
```

This will create a file in the output directory (default: "output") containing the contents of all files in the specified folder, with each file's content prefixed by its name and path.

Options:

- `-t, --timeout <SECONDS>`: Set the timeout for the git clone operation (default: 5 seconds)
- `-o, --output-dir <DIR>`: Specify the output directory (default: "output")

For more information, use the `--help` flag:

```
copyrepo --help
```

## Output Format

The tool generates a single text file with the following format for each file in the specified GitHub folder:

```
File: path/to/file1.ext
[Contents of file1]

File: path/to/file2.ext
[Contents of file2]

...
```

This format ensures that LLMs can easily distinguish between different files and their contents, preserving the structure of the original GitHub folder.

## License

This project is licensed under the MIT License. See the [LICENSE](https://github.com/vacekj/copyrepo/blob/main/LICENSE) file for details.

## Author

Josef Vacek

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request on the GitHub repository: [https://github.com/vacekj/copyrepo](https://github.com/vacekj/copyrepo)

## Issues

If you encounter any problems or have suggestions, please file an issue on the GitHub issue tracker: [https://github.com/vacekj/copyrepo/issues](https://github.com/vacekj/copyrepo/issues)