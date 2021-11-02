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

We took some inspiration from
[rooster](https://github.com/conradkleinespel/rooster), which is at about 4k
lines of code. We want our password manager to stay well below that. For that
we will opt for a minimal design, including only basic features.

## Features

* Entries have an entry name, a description, a username, an email, other notes,
and a password as fields;
* List entries;
* Add new entry (user is prompted about the entry fields);
* Retrieve entry (user types the entry name, a search is performed and results
shown, password is stored in the clipboard);
* Passwords should not be visible when user is typing.

## Commands and Usage

The main commands are:

    $ kapa                   # creates a new data file, or, if it already
                             # exists, displays the help text

    $ kapa ls                # lists entry names
    $ kapa <entry_name>      # gets entry with specified name
    $ kapa add <entry_name>  # adds entry with specified name
    $ kapa rm <entry_name>   # removes entry with specified name

Backups can be done by copying the password file. We might implement an export
function to export everything unencrypted to a JSON, TOML or YAML file.

# Cryptography

We will use AES-256 in CBC mode for encryption. An initialization vector (IV) is
randomly generated and stored with each message/entry. For the moment, we have
no plans to use MAC or anything for authentication -- i.e. you will not be able
to tell if the data has been tampered with.

