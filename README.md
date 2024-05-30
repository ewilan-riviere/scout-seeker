# Scout

![Banner with large array and scout title](https://raw.githubusercontent.com/ewilan-riviere/scout/main/docs/banner.jpg)

[![rust][rust-version-src]][rust-version-href]
[![tests][tests-src]][tests-href]

<!-- [![codecov][codecov-src]][codecov-href] -->

Scout is a simple [Rust](https://www.rust-lang.org/) CLI to scan a directory to list files, recursively.

## Download

You can download `scout` from the [releases page](https://github.com/ewilan-riviere/scout/releases/latest).

## Usage

You have to pass the directory to scan as an argument.

Optional arguments:

- `-o` or `--output`: to specify the output file, by default it will be `./output.json`.

```bash
scout /path/to/directory -o=/path/to/output.json
```

You will have an output, like this:

```bash
Directory: /path/to/directory
Date: "2024-04-28 10:07:12.081879 +02:00"
Time in seconds: 3.123456
Total files: 1234
Output file: /path/to/output.json
```

And an output file, like this:

```json
{
  "path": "/path/to/directory",
  "date": "2024-04-28 10:07:12.081879 +02:00",
  "time_seconds": "3.123456",
  "total_files": 1234,
  "files": ["/path/to/directory/file1.txt", "/path/to/directory/file2.txt"]
}
```

### Files excluded

- Files with dots at the beginning of their names, like `.gitignore`.
- Files into directories with dots at the beginning of their names, like `.git/HEAD`.

## Build

Clone repository:

```bash
git clone https://github.com/ewilan-riviere/scout.git
```

To build, you have to install [Rust](https://www.rust-lang.org/), you can follow [this guide](https://gist.github.com/ewilan-riviere/6a0b8aab2e347164e73feab83c862e99).

```bash
cargo build
```

```bash
cargo run /path/to/directory
```

## Release

Build the release version:

```bash
cargo build --release
```

Put the binary in your path:

```bash
cp target/release/scout /usr/local/bin
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

[rust-version-src]: https://img.shields.io/badge/Rust-v1.77.2-000000?colorA=18181B&logo=Rust&logoColor=ffffff
[rust-version-href]: https://www.rust-lang.org/
[tests-src]: https://img.shields.io/github/actions/workflow/status/ewilan-riviere/scout/run-tests.yml?branch=main&label=tests&style=flat&colorA=18181B
[tests-href]: https://github.com/ewilan-riviere/scout/actions
[codecov-src]: https://img.shields.io/codecov/c/gh/ewilan-riviere/scout/main?style=flat&colorA=18181B&colorB=777BB4
[codecov-href]: https://codecov.io/gh/ewilan-riviere/scout
