** This is a work in progress. Please do not trust it with your passwords. **

A password manager for the terminal written in Rust.

Main goal is simplicity:

* other alternatives like 
[rooster](https://github.com/conradkleinespel/rooster) are above 4k lines of
code, we want to stay well below that;
* just the basic features: ability to add and retrieve passwords.

Features:

* Entries have entry name, description, username, password, email, app, website
and other info as fields;
* Add new entry (user is prompted about the entry fields);
* List entries;
* Retrieve entry (user types the app/website name, a search is performed and
results shown, password is stored in the clipboard);
* Password typing should use shadowing (`*****`).

Implementation ideas:

Never decrypt things that are not needed. Decrypted information should be
deleted (from memory, i.e., go out of scope) as soon possible -- right after
use. All entry names/descriptions (or searchable fields) should be stored 
together and decrypted at once for searchability. Once the search is done and
entry is selected, only the desired entry is decrypted.

Experience should be more or less like the following. Retrieving a password:

    $ password-manager get github
    Please enter your master password:
    *******

    Entry: github
    description: Github account at github.com
    user: myname@email.com
    pw: ******* (copied to your clipboard, paste to use)

Adding a password:

    $ password-manager add github
    Please enter the description:
    Github account at github.com

    Please enter the email:
    myname@email.com

    Please enter the password for this entry:
    ******

    Please enter the additional information:


    Please enter your master password:
    *******

The user can just press ENTER upon a prompt to leave a field blank.

Backups can be done by copying the password file. We might implement an export
function to export everything unencrypted to a JSON, TOML or YAML file.

