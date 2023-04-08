# brain-cli

This is a rust cli (command line interface) program I developed to let users save things they need to reference later.

This is the 🥈`second (v2)` slightly less bare bones version. 

# What it looks like:
![](./brainv2v2.gif)

## Usage/Features:
Invoke the Brain with `brain`
^ To install this cli on your computer see [below instructions](#how-to-add-this-binary-to-your-path-in-macos-and-linux)
```
brain
🧠 (Brain): What do you want to remember?
....{your response}..
```
^ This saves the contents to `./.brain-cli/brain.md` in the directory where `brain` binary is running from.

Other commands:
```
# list the brain file in the terminal (with some minimal markdown highlighting)
brain list

# opens your brain file with your default editor. Under the hood uses: `open -e ./brain-cli/brain.md`. Works on Macos, but not sure on other platforms. 
brain open

```

# Why?
I find there are many things I want to remember, but not a super easy way to just dump them somewhere formatted nicely. 
So this is an attempt at just dumping things I need to refer back to later.

## Run with Cargo Locally
**Pre requisite:** Need to have rust/cargo installed [instructions here](https://doc.rust-lang.org/book/ch01-01-installation.html)

```
git clone https://github.com/goshipcode/brain-cli.git
cd brain-cli
cargo run 
```

## How to add this binary to your path in macos and linux
```
# First create a release version of the binary
cargo build --release

# Now copy the binary to a bin directory of your choice:
cp target/release/brain [directory of your choice]/bin
chmod 755 [directory of your choice]/bin/brain

# Add the binary to your path
export PATH="[directory of your choice]/bin:$PATH"

# Now you can execute `brain` from your terminal:
brain 
```

## Credits
Thank you, @Canop for creating a [rust markdown rending on terminal crate](https://github.com/Canop/termimad). I used this for pretty rendering of the brain.md file on the terminal.
