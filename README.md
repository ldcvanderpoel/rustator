# Rustator

A tool for aiding subdomain discovery via generation of alternative subdomains by combining lists of subdomains, keywords, and separators.


In other words, given the subdomains:
```
server
mail
```

And the keywords:
```
acc
prod
```

And the seperators:

```
-
.

```

We generate
```
server
mail
acc.server
server.acc
acc-server
server-acc
accserver
serveracc
prod.server
server.prod
prod-server
server-prod
prodserver
serverprod
acc.mail
mail.acc
acc-mail
mail-acc
accmail
mailacc
prod.mail
mail.prod
prod-mail
mail-prod
prodmail
mailprod
```

# Installation

Download the Git repository and use Cargo to build the package:
```
cargo build --release
```
# Usage


## Combine
```
Combines subdomains and keywords using separators

Usage: rustator combine <SUBDOMAINS> <KEYWORDS> <SEPARATORS>

Arguments:
  <SUBDOMAINS>  File containing a list of subdomains
  <KEYWORDS>    File containing a list of keywords
  <SEPARATORS>  File containing a list of seperators

Options:
  -h, --help  Print help information
```

### Example
```
$PWD/target/release/rustator combine testing/subdomains.txt testing/keywords.txt testing/separators.txt
server
mail
acc.server
server.acc
acc-server
server-acc
accserver
serveracc
prod.server
server.prod
prod-server
server-prod
prodserver
serverprod
acc.mail
mail.acc
acc-mail
mail-acc
accmail
mailacc
prod.mail
mail.prod
prod-mail
mail-prod
prodmail
mailprod
```

# Testing

```
cargo test
```

# Inspiration
This tool was heavily inspired by [Gotator](https://github.com/Josue87/gotator) and [altDNS](https://github.com/infosec-au/altdns). The design is slightly differend however, aiming for simply creating a way to combine wordlists. This requirement was based from observations of DNS records in the wild.
