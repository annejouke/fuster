# Fuster

A personal assistant to keep the hen house in order

## Features

To do

- [] `fus rocket` command to push the vital "🚀 initial commit" to the clipboard (humble beginnings...)
- [] See if Github Actions can be used to build this for Windows, Linux and macOS
- [] Extend the `README.md` with a shell script example on how to install this to the PATH
- [] `fus project standardize` command to set your project formatting defaults as set by your `~/.fus/config.toml` and `~/.fus/standards/**/*` settings

## Project Standardise

Some features of `fus project standardize`:
- It'll check the contents of the folder and sub directories for the known culprits such as `package.json` `cargo.toml` and `go.mod` to decide the project type
- On finding it, it'll initialise the project with the standardised settings as set in `~/.fus/config.toml` and `~/.fus/standards/**/*`
- Based on this information it'll copy the appropriate `.gitignore` to the project root
- It'll take all files in the root of the `~/.fus/standards/default/**/*` folder and copy them to the project root
- It'll merge/override all files in the specific project's flavor folder `~/.fus/standards/<project_type>/**/*` to the project root: Types and names to be determined
