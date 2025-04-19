# Zettelviewer

A lightweight, memory-efficient Markdown viewer specifically designed for Zettelkasten note collections.

## Overview

Zettelviewer is a simple web-based tool that renders Markdown files from a specified directory without writing any HTML files to disk. It's particularly useful for quickly browsing through Zettelkasten-style notes in a clean, formatted interface.

Key features:
- Renders Markdown files to HTML in memory (no files written to disk)
- Displays file names alongside their titles (from H1 headings)
- Clean, readable interface with proper formatting for code blocks
- Built-in web server for easy viewing in any browser
- Support for common Markdown extensions (tables, footnotes, strikethrough, task lists)

## Installation

### Prerequisites
- Rust and Cargo (https://www.rust-lang.org/tools/install)

### Building from source
```bash
# Clone the repository
git clone https://github.com/yourusername/zettelviewer.git
cd zettelviewer

# Build in release mode
cargo build --release

# The binary will be available at target/release/zettelviewer
```

## Usage

```bash
# Start the server with the path to your Markdown folder
./zettelviewer path/to/your/markdown/folder
```

Then open `http://localhost:3030` in your web browser to view your notes.

## Features

### Supported Markdown Features
- Headers (H1-H6)
- Emphasis (bold, italic)
- Lists (ordered and unordered)
- Code blocks with syntax highlighting
- Tables
- Blockquotes
- Links
- Images
- Footnotes
- Strikethrough
- Task lists

### Zettelkasten Integration
Zettelviewer is designed with Zettelkasten note-taking methodology in mind:
- File names are displayed together with their titles for easy reference
- Clean interface allows for distraction-free reading
- In-memory rendering respects the plain text philosophy of Zettelkasten

## Customization

You can customize the HTML templates and styling by modifying the source code. Look for the HTML template strings in `src/main.rs`.

## Development

### Dependencies
- `pulldown-cmark`: For Markdown parsing
- `warp`: For the web server
- `tokio`: For async runtime

### Building for development
```bash
cargo build
cargo run -- path/to/your/markdown/folder
```

### Project Structure
- `src/main.rs`: Contains all the code for the application

## Contributing

Contributions are welcome! Feel free to submit issues or pull requests.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- Built using Rust and several awesome crates
- Inspired by the Zettelkasten method of note-taking
