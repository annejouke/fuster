# Fuster Cluck

A personal assistant to keep the clucking hen house in order

!["Fuster Cluck"](./img/fuster.png "Fuster Cluck")

## Installation

For Windows:

```<todo>```

For macOS:

```curl https://github.com/annejouke/fuster/blob/main/install-macos.sh | sh```

For Linux:

```<todo>```

### Features

- Use `fus rocket` to print the one and only proper way to format your initial commit to the clipboard
- Actually useful features coming soon...

### To Do List

- [x] `fus rocket` command to push the vital "🚀 initial commit" to the clipboard (humble beginnings...)
- [x] See if Github Actions can be used to build this for Windows, Linux and macOS
- [x] Extend the `README.md` with a shell script example on how to install this to the PATH
- [ ] Include Linux and Windows install instructions
- [ ] `fus init` command to set your project formatting defaults as set by your `~/.fus/config.toml` and `~/.fus/init/**/*` settings

### Project Standardise

> WIP!

Some features of `fus init`:
- It'll check the contents of the folder and sub directories for the known culprits such as `package.json` `cargo.toml` and `go.mod` to decide the project type
- On finding it, it'll initialise the project with the standardised settings as set in `~/.fus/config.toml` and `~/.fus/init/**/*`
- Based on this information it'll copy the appropriate `.gitignore` to the project root
- It'll take all files in the root of the `~/.fus/init/default/**/*` folder and copy them to the project root
- It'll merge/override all files in the specific project's flavor folder `~/.fus/init/<project_type>/**/*` to the project root: Types and names to be determined

## About

This CLI was created because I wanted to and I like writing Rust and using `clap` (it's just awesome). I don't perceive it'll be useful for anyone, but if it is, feel free to use it, contribute to it or make suggestions on how it should be changed.
