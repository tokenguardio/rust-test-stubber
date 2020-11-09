# Blockhunters Rust Test Stubber [ALPHA]

This is an extension that allows to generate test stubs in your Rust project.

## Description

This is a VS Code extension that allows to generate high quality test stubs in your Rust project. 
It generates stubs for functions, class methods and traits. 
Each stub contains all required elements such as test methods and mock implementations of injected traits with respect to default Rust formatting.

We suggest to generate the stubs once the whole part of code is written in a file. The extension doesn't have parsing functionality yet so it won't be able to add additional tests to new parts of the code in a file. 

Please note that this is a beta version and has limited functionality that will be developed gradually.

## Installation guide

1. Install this plugin using VSCode Marketplace.
2. Clone this project using
   ```shell
   $ git clone https://github.com/blockhunters/rust-test-stubber.git
   ```
3. Go to the root directory of that repository and run `cargo install` in console.
4. Make sure that `~/.cargo/bin` is in your PATH variable.
5. You can now run the stubber using command `Rust Test Stubber: Create Stubs`

## Contact

This extension is brought to you by [https://blockhunters.io](https://blockhunters.io/). Feel free to contact us at [konrad.kleczkowski@blockhunters.io](mailto:konrad.kleczkowski@blockhunters.io)