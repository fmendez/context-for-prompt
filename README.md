# context-for-prompt

Creates a context made of all the files and their content. The content is copied to the clipboard.

## Installation

```sh
cargo install context-for-prompt
```

## Usage

```sh
Creates a context made of all the files and their content.

Usage: context-for-prompt [OPTIONS] <ROOT>

Arguments:
  <ROOT>  

Options:
  -e, --extensions-to-ignore <EXTENSIONS_TO_IGNORE>  
  -d, --debug                                        
      --hidden                                       
  -h, --help                                         Print help
  -V, --version                                      Print version
```

```sh
context-for-prompt /path/to/directory
```
The goal of this program is to walk through a directory and copy the content of each file to the clipboard. The file content should be copied in the following format:

```
file: /path/to/file
----------- content start -------------
file content
----------- content end -------------
```

This content can then be used as context for LLMS prompts.

The search will ignore hidden files by default and anything contained in the `.gitignore` file. You can use the `--hidden` flag to include hidden files and directories. You can use the `--extensions-to-ignore` flag to ignore files with certain extensions. For instance, if you want to ignore `.md` files, you can use the following command:

```sh
context-for-prompt /path/to/directory -e=md,lock 
``` 

Note: This crate was inspired by https://github.com/simonw/files-to-prompt