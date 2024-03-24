# A random password generator

## Overview
A command line tool to generate passwords. This tool has the flexibility of specifying a minimum amount of a particular character set (i.e. alphabet, numeric, or special).

## Usage
```
Usage: pwdgen.exe [OPTIONS]

Options:
  -l, --length <LENGTH>        Length of the password. Will be overriden if less than the sum of minimum characters. [default: 16]
      --upper <MIN_UPPER>      Minimum uppercase characters. [default: 0]
      --lower <MIN_LOWER>      Minimum lowercase characters. [default: 0]
  -a, --alpha <MIN_ALPHA>      Minimum alphabet characters. Will override either upper or lower. [default: 0]
  -n, --numeric <MIN_NUMERIC>  Minimum numeric characters. [default: 0]
  -s, --special <MIN_SPECIAL>  Minimum special characters. [default: 0]
      --show                   Prints the password after generating.
      --copy                   Copies the generated password to clipboard.
  -v, --verbose                Prints verbose output.
  -h, --help                   Print help
```

## Notes

With no arguments provided, the default password length is 16, and generates using alphanumeric characters.
