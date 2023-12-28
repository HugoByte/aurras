# Composer-v2

[![License: Apache-2.0](https://img.shields.io/github/license/icon-project/IBC-Integration.svg?style=flat-square)](https://www.apache.org/licenses/LICENSE-2.0)

## Introduction

The Composer is an integrated software package, encompassing both the Echo Library and Echo CLI components. This comprehensive solution is specifically tailored to streamline the process of generating WebAssembly (Wasm) files. The Echo Library serves as a foundational building block, offering a rich set of functionalities, while the Echo CLI excels in orchestrating the creation of Wasm files based on a specified list of configuration files. This sophisticated combination empowers developers by providing a seamless and organized approach to translating configuration parameters into fully functional web applications, enhancing the overall development experience.

 The Echo-Library and Echo-Cli tandem empower developers with a comprehensive solution for defining, managing, and executing workflows with ease. By harnessing Rust's capabilities, these tools provide a solid foundation for creating efficient and optimized WebAssembly files, offering developers a versatile toolkit to streamline their development processes.

## Prerequisite

- Ensure [Rust](https://www.rust-lang.org/tools/install) is installed and updated to the latest version.
  
## Getting started

- Clone the repository
  
  ```
  git clone https://github.com/HugoByte/internal-research-and-sample-code.git
  ```

- change the directory to `composer-dev`

- Installing the build-libraries
  
  ```
  brew install llvm@11  
  ```

  ```
  export CC=/opt/homebrew/Cellar/llvm@11/11.1.0_4/bin/clang-11 && export AR=/opt/homebrew/Cellar/llvm@11/11.1.0_4/bin/llvm-ar
  ```
  *Note: This is required only for polkadot*

- Installing the echo-library

  ```
  cargo install --path package
  ```

- Run
  
  ```
  composer
  ```

## Usage

- Building the current package
  
  ```
  composer build
  ```

- Validating the config file
  
  ```
  composer validate
  ```

- Creating the new Package
  
  ```
  composer create <package_name>
  ```

## Example

- Validating the config file
  
    ```
    composer validate ./example
    ```

- Build the config file

    ```
    composer build ./example -o ./example
    ```

## License

Licensed under [Apache-2.0](https://www.apache.org/licenses/LICENSE-2.0)