<div align="center">
![banner](./art/banner.png)
[![GitHub release](https://img.shields.io/github/release/riviox/flash.svg)](https://github.com/riviox/flash/releases)
[![License](https://img.shields.io/github/license/riviox/flash.svg)](https://github.com/riviox/flash/blob/main/LICENSE)
[![GitHub issues](https://img.shields.io/github/issues/riviox/flash.svg)](https://github.com/riviox/flash/issues)
[![GitHub stars](https://img.shields.io/github/stars/riviox/flash.svg)](https://github.com/riviox/flash/stargazers)
![GitHub code size in bytes](https://img.shields.io/github/languages/code-size/riviox/flash)
# ⚡flash⚡
A lightweight package manager for managing GitHub-based packages. Install, list, remove, and update packages with ease directly from the command line.
</div>

## Usage

Flash supports the following commands:

<img src="./art/logo.png" alt="Flash Logo" align="right" height="200px">

- **Install a Package**: `-S <github_user>/<repo>`
- **List cloned Packages**: `-L`
- **Remove a Package**: `-R <package>`
- **Update a Package**: `-U <package>`
- **Update All Installed Packages**: `-UA`

# Package template:

- `exec`: Specifies the installation command for the package.
- `name`: Provides the name of the package.
- `desc`: Offers a brief description of the package.
- `clear=true/false`: Indicates whether the cloned package directory should be removed after installation.
## Example:
```
exec=make install
name=flash
desc=A lightweight package manager for managing GitHub-based packages. Install, list, remove, and update packages with ease directly from the command line.
clear=true
```

![ss](https://github.com/riviox/flash/assets/100956266/d3f00bde-6030-4996-a25e-d8cd9c259e0c)
