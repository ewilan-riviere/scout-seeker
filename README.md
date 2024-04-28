# Scout

[![rust][rust-version-src]][rust-version-href]
[![tests][tests-src]][tests-href]

<!-- [![codecov][codecov-src]][codecov-href] -->

Scout is a simple [Rust](https://www.rust-lang.org/) CLI to scan a directory to list files, recursively.

## Usage

```bash
cargo build
```

```bash
cargo run /path/to/directory
```

## Output

```bash
Directory: /path/to/directory
Time in seconds: 3.123456
Total files: 1234
Output file: /path/to/output.json
```

### JSON output

```json
{
  "path": "/path/to/directory",
  "time_seconds": "3.123456",
  "total_files": 1234,
  "files": ["/path/to/directory/file1.txt", "/path/to/directory/file2.txt"]
}
```

## Release

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
