# A random password generator

## Overview
A command line tool to generate passwords. This tool has the flexibility of specifying a minimum amount of a particular character set (i.e. alphabet, numeric, or special).

## Usage
```
Usage: pwdgen.exe [OPTIONS]

Options:
  -l, --length <LENGTH>        Length of the password. Will be overriden if less than the sum of minimum characters. [default: 16]
  -a, --alpha <MIN_ALPHA>      Minimum alphabet characters. [default: 1]
  -n, --numeric <MIN_NUMERIC>  Minimum numeric characters. [default: 1]
  -s, --special <MIN_SPECIAL>  Minimum special characters. [default: 1]
  -h, --help                   Print help
```