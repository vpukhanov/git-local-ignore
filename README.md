# git-local-ignore

This command-line utility lets you manipulate .git repository's `info/exclude` file, which works like `.gitignore`
but is completely local and not included in the repository itself. 

[Learn more about .git/info/exclude](https://git-scm.com/docs/gitignore)

## Installation

    cargo install git-local-ignore
    
Or using `brew` on macOS:

    brew install vpukhanov/tools/git-local-ignore
    
## Usage

    # Add files to exclude list
    git-local-ignore filename1 filenam2
    
    # Add multiple files to exclude list using glob pattern
    git-local-ignore filename*.txt
    
    # Add glob pattern itself to exclude list
    git-local-ignore filename\*.txt
    
    # Display entries in the exclude list
    git-local-ignore --list
    
    # Clear the exclude list
    git-local-ignore --clear
    
    # Display help
    git-local-ignore--help