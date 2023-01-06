# defederate.me

This repo contains the files to create a table like the one seen on 
https://defederate.me/#blocklist.

## Usage

1. Create a double-bar-speparated-values file named `blocklist_[id].dbsv` and
   write your blocklist.
2. Run `cargo run [id] | xclip -selection clipboard`
3. You now have some HTML in your clipboard.

## Flaws + TODO

- We are currently using an async runtime for no apparent reason, but this is
  just to be able to expand the codebase later and add more stuff.
- We want to check if a domain still exists before displaying it to the main
  site.

## What is this for?

Converting a stupid file (DBSV) to a smart file (HTML + JS).

## Can I use it?

Yes, but it must be according to AGPL.
