<div align="center">
  <img src="./art/banner.png" alt="banner">
  <br>
  <a href="https://github.com/riviox/flash/releases"><img src="https://img.shields.io/github/release/riviox/flash.svg" alt="GitHub release"></a>
  <a href="https://github.com/riviox/flash/blob/main/LICENSE"><img src="https://img.shields.io/github/license/riviox/flash.svg" alt="License"></a>
  <a href="https://github.com/riviox/flash/issues"><img src="https://img.shields.io/github/issues/riviox/flash.svg" alt="GitHub issues"></a>
  <a href="https://github.com/riviox/flash/stargazers"><img src="https://img.shields.io/github/stars/riviox/flash.svg" alt="GitHub stars"></a>
  <img src="https://img.shields.io/github/languages/code-size/riviox/flash" alt="GitHub code size in bytes">
  <h1>⚡flash⚡</h1>
  <p>A lightweight package manager for managing GitHub-based packages. Install, list, and update packages with ease directly from the command line.</p>
</div>

## Install:
```bash
curl -sSL https://riviox.is-a.dev/flash.sh | bash
```
or
```bash
curl -sSL https://raw.githubusercontent.com/riviox/flash/master/installer.sh | bash
```

## Usage

Flash supports the following commands:

<img src="./art/logo.png" alt="Flash Logo" align="right" height="200px">

- **Install a Package**: `-S <github_user>/<repo>`
- **List cloned Packages**: `-L`
- **Update a Package**: `-U <package>`
- **Update All Installed Packages**: `-UA`

# Package template:

- `exec`: Specifies the installation command for the package.
- `name`: Provides the name of the package.
- `desc`: Offers a brief description of the package.
- `clear=true/false`: Indicates whether the cloned package directory should be removed after installation.
- `deps`: Specifies dependencies
## Example:
`config.flash`
```
exec=make install
name=flash
desc=A lightweight package manager for managing GitHub-based packages. Install, list, remove, and update packages with ease directly from the command line.
clear=true
deps=rust
```

![ss](https://github.com/riviox/flash/assets/100956266/d3f00bde-6030-4996-a25e-d8cd9c259e0c)
