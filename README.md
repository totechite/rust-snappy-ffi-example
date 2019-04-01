# rust-snappy-example
It's program that bind to Snappy implemented by Rust language.    
Reference to [chapter of FFI](https://doc.rust-lang.org/nomicon/ffi.html) in The Rust Programming Language(TRPL).

Usage
--------------------
### 1. Install Snappy
#### Linux
- ubuntu
```bash
sudo apt install libsnappy-dev
```
Other Linux distributions can be like this.   

#### Windows
- install [Vcpkg](https://github.com/Microsoft/vcpkg)   

- Open PowerShell and execute ```vcpkg install snappy:x64-windows``` or ```vcpkg install snappy:x86-windows```.   

- Edit build.rs. Rewrite ```<path_to_vcpkg>``` to your Path of Vcpkg.   

### 2. Execute
```bash
cargo run
```   
Please reference to [https://doc.rust-lang.org/nomicon/ffi.html]().

LICENSE
--------------------
MIT

