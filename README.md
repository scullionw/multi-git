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