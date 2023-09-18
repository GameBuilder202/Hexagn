# Hexagn: Higher-up for URCL

![GitHub top language](https://img.shields.io/github/languages/top/GameBuilder202/Hexagn?color=7047EF&style=flat-square)
![GitHub repo size](https://img.shields.io/github/repo-size/GameBuilder202/Hexagn?color=7047EF&style=flat-square)
![GitHub commit activity](https://img.shields.io/github/commit-activity/m/GameBuilder202/Hexagn?color=7047EF&style=flat-square)
![Discord](https://img.shields.io/discord/1015615296939233340?color=7047EF&style=flat-square)

A simple compiler capable of compiling programs written in the Hexagn programming language to URCL.
The Hexagn language with its C-like syntax allows anyone to easily understand it.

<p align="center"><img alt="Hexagn banner" src="./banner.png"></p>

*Preview of Hexagn's logo and syntax; Outputting "Hello, GitHub!"*

## Prerequisites

If you are running a Windows NT system, WSL (Windows Subsystem for Linux) is recommended. Although it should work in NT systems, it is not tested.<br>
However, if you are using MacOS or (any) Linux (distro), you can continue the installation process.

It is recommended to install the Rust programming language via ([Rustup](https://rustup.rs))

- Rust programming language
- `git` or `gh` ([Github CLI](https://cli.github.com/)) <br> *\* not required if you download the repository via Web or Desktop*

## Installation (unstable)

Currently, there are no stable releases of Hexagn, cloning the repository directly will give you the in-development or unstable version.
Proceed if you are willing to face consequences.

#### CLONING, BUILDING AND INSTALLING

1. Clone the repository via any option in the "**<> Code**" tab, which can be found on the top of the GitHub repository's page.
2. Run the following command in the `./Hexagn/` directory (The cloned repository, replace if you cloned under a different name).
```sh
$ cargo build --release; cargo install --path .
```
3. Hexagn should be compiled as an executable inside the `./target/release/` directory and installed to your Cargo binaries.
4. You should now be able to use the command `hexagn` anywhere, as long it is within the same shell.

## Executable details

\* *`<>` Denotes required, `[]` Denotes optional.*

```
hexagn <INPUT FILE> [FLAGS]
```

#### BUILT-IN COMMANDS

`-h`, `--help`: Highlights available options and parametres.<br>

#### BUILT-IN FLAGS

`-o <OUTPUT FILE>`: Overwrites which output file to compile to. (Default: `out.urcl`)<br>
`-l <LIBRARY PATH>`: Includes library path in compiling phase.<br>
`-O <LEVEL>`: Selects a specific optimisation level.<br>
`-g`: Enables debug symbols.<br>
`--no-main`: Removes entry-point on compiled file, Rendering the code as a library.

## Contributing

You can support the project by contributing to it via forking the repository, commit changes and opening up a pull request (Instructions on how to fork it can be read [here](https://docs.github.com/en/get-started/quickstart/contributing-to-projects)), leaving a feedback or reporting an issue.

After a pull request, please be patient for maintainers to review and manage conflicts.

## Resources

### Official site

The GitHub pages site, `notalternate.github.io` is still in progress, Check here again after a few days.

### Documentations

Hexagn documentations are still in progress, Check here again after a few days.

### Discord server

Did you know that Hexagn has its own dedicated Discord server?<br>
Join and check it out from [here](https://discord.gg/invite/t75crS5XBe).

**You are requried to obey the set guidelines in-order to interact within the server.
Server guidelines are subjected to change, check announcements for guidelines change.
Please discuss Hexagn related matters within specific channels. Further guidelines exists.**

## Repository license

The Hexagn compiler repository owned by GameBuilder202 and NotAlternate is licensed under the [MIT](LICENSE) license.