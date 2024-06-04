# PEAR and PECL Client

[![GitHub](https://img.shields.io/badge/GitHub-demurgos%2Fpear--client-informational.svg?maxAge=86400)](https://github.com/demurgos/pear-client)
[![crates.io](https://img.shields.io/crates/v/pear_client.svg?maxAge=86400)](https://crates.io/crates/pear_client)
[![CI status](https://img.shields.io/github/actions/workflow/status/demurgos/pear-client/check-rs.yml.svg?branch=main&maxAge=86400)](https://github.com/demurgos/pear-client/actions/workflows/check-rs.yml?query=branch%3Amain)
[![docs.rs/pear_client](https://img.shields.io/docsrs/pear_client.svg?maxAge=86400)](https://docs.rs/pear_client)
[![license MIT](https://img.shields.io/badge/license-AGPL--3.0--or--later-green)](./LICENSE.md)

Client for PHP Extension and Application Repository (PEAR) registries, including the PHP Extension Community Library (PECL).

## Installation

Run the following command in your project:
```
cargo add pecl_client
```

## Usage

See examples directory.

This library is organized as a set of [`tower_service`](https://docs.rs/tower-service) handlers.
This allows to abstract the transport layer and focus on the PECL API.

# Documentation

See [docs.rs/pecl_client](https://docs.rs/pecl_client).

# Maintenance status

This library is incomplete for now. Development is not a high priority. New APIs are added as needed.

If you want to improve the library, feel free to open an issue or send a PR. Breaking changes are allowed.
Note however that review times may be slow.

# Reference

- [PEAR channel server REST interface](https://pear.php.net/manual/en/core.rest.php)

# License

[AGPL-3.0-or-later](./LICENSE.md)
