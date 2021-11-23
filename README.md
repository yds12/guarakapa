![tests](https://github.com/yds12/guarakapa/actions/workflows/build_and_test.yml/badge.svg)

**This is a work in progress. Please do not trust it with your passwords.**

A password manager for the Linux (X11) terminal written in Rust.

# Install and Run

Install dependencies (`openssl`, `libxcb-shape` and `libxcb-xfixes`).
For Ubuntu:

    $ apt install libssl-dev libxcb-shape0-dev libxcb-xfixes0-dev

Install using cargo:

    $ git clone https://github.com/yds12/guarakapa
    $ cd guarakapa
    $ cargo install --path .

And run:

    $ kapa

Run tests with:

    $ cargo test -- --test-threads=1               # all tests
    $ cargo test --bins --lib                      # unit tests
    $ cargo test --test '*' -- --test-threads=1    # integration tests

For now, we have to avoid running integration tests in parallel because they
all manipulate the same data file.

# About

This project started with the purpose of learning Rust, how to use its testing
tools, and the basics of cryptography.

It is a command line program for Linux that enables you to save your passwords
and retrieve them (using a master password) directly via the clipboard.

The core design principles are:

* **Minimalism:** keep the source code, number of dependencies, binary size and
compilation time as small as possible;

* **Security:** use standard highly-secure cryptography algorithms, only decrypt
what is necessary at a given moment, keep track of the version used to create
the password file, extensive use of tests;

* **Usability:** we strive to make it as easy as possible to use in a terminal.

We took some inspiration from
[rooster](https://github.com/conradkleinespel/rooster).

## Features

* Entries have an entry name, a description, a username, an email, other notes,
and a password as fields;
* List entries: only entry names are decrypted, not the entries themselves;
* Add new entry: user is prompted about the entry fields;
* Retrieve entry: user types the entry name, the selected entry is shown,
password is stored in the clipboard;
* Passwords should not be visible when user is typing;
* Find your password file;
* Find which version of the program was used to create your password file, so
that in case there is a breaking change you can still recover it with an older
version.

## Commands and Usage

The basic commands are:

    $ kapa                   # creates a new data file, or, if it already
                             # exists, displays the help text

    $ kapa ls                # lists entry names
    $ kapa <entry_name>      # gets entry with specified name
    $ kapa add <entry_name>  # adds entry with specified name

Learn more about all the commands and options with:

    $ kapa --help

Backups can be done by copying the password file (find it with `kapa path`).
We might implement an export function to export everything unencrypted to a
JSON, TOML or YAML file.

# Cryptography

We will use AES-256 in CBC mode for encryption. An initialization vector (IV) is
randomly generated and stored with each message/entry. For the moment, we have
no plans to use MAC or anything for authentication -- i.e. you will not be able
to tell if the data has been tampered with.

