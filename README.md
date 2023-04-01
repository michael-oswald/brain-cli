# brain-cli

This is a rust cli (command line interface) program I developed to let users save things they need to remember later.

This is the first v1 `very bare bones version` that:
1. creates a brain.md file where the executable is run
2. Stores your remember sakes in that file
3. Has the ability to list your memories

# Why?
I find there are many things I want to remember, but not an easy way to just dump them somewhere formatted nicely. 
So this is an attempt at just dumping things I need to refer back to later. I plan to make iterations on this idea
and add more features. 

For now just a simple project I have developed to help me learn rust better.

# What it looks like:
![](./brainv1.gif)

## Installation
**Prereq:** Need to have rust/cargo installed [instructions here](https://doc.rust-lang.org/book/ch01-01-installation.html)

```
git clone https://github.com/goshipcode/brain-cli.git
cd brain-cli
cargo run 
```

## How to add this binary to your path in macos and linux
```
# first create a release version of the binary
cargo build --release

# Now copy the binary to 
cp target/release/brain [directory of your choice]/bin
chmod 755 [directory of your choice]/bin/brain

# Add the binary to your path
export PATH="[directory of your choice]/bin:$PATH"
```

