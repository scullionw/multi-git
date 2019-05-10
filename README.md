# multi-git
[![Crates.io](https://img.shields.io/crates/v/multi-git.svg)](https://crates.io/crates/multi-git)

![](demo/demo.gif)

    C:\Users\LUNA>mgit --help
    multi-git 0.1.0
    scullionw <williamscullion@gmail.com>
    Quick CLI to view git status of projects

    USAGE:
        mgit [target_dir]

    FLAGS:
        -h, --help       Prints help information
        -V, --version    Prints version information

    ARGS:
        <target_dir>
# Installation
Crates.io:

        cargo install multi-git
        
or latest from git:

        cargo install --git "https://github.com/scullionw/multi-git"
        
or from source:

        cargo build --release
        sudo chmod +x /target/release/mgit
        sudo cp /target/release/mgit /usr/local/bin/


    
# Example script on windows (check.bat):
    mgit "C:\Users\User\Desktop\Projects"
    pause

# Example script on macOS (chmod +x check.sh):
    #!/bin/bash
    mgit "/Users/username/Projects"
    read -n 1 -s -r -p "Press any key to continue"