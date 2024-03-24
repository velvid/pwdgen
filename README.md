# A random password generator

## Overview
A command line tool to generate passwords. Can specify the minimum characters required

## Usage
```
Usage: pwdgen.exe [OPTIONS]

Options:
  -l, --length <LENGTH>          Length of the password. Will be overriden if less than the sum of minimum characters. [default: 16]
  -a, --alpha [<MIN_ALPHA>]      Minimum alphabet characters. If flag is not set, won't be added to character pool.
  -n, --numeric [<MIN_NUMERIC>]  Minimum numeric characters. If flag is not set, won't be added to character pool.
  -s, --special [<MIN_SPECIAL>]  Minimum special characters. If flag is not set, won't be added to character pool.
      --show                     Prints the password after generating.
      --copy                     Copies the generated password to clipboard.
  -v, --verbose                  Prints information such as time taken to generate password and character pool distribution.
  -h, --help                     Print help
```

The `alpha`, `numeric`, and `special` flags don't require a following argument. The presence of the flag implies to add the character set to the pool of characters to sample from, with a minimum of 0 (not required to be in the final password).

If no flags for character sets are provided, will default to alphanumeric characters.
