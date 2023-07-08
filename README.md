# stac-server-rs

[![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/gadomski/stac-server-rs/ci.yaml?branch=main&style=for-the-badge)](https://github.com/gadomski/stac-server-rs/actions/workflows/ci.yaml)
[![STAC API Validator](https://img.shields.io/github/actions/workflow/status/gadomski/stac-server-rs/validate.yaml?branch=main&label=STAC+API+Validator&style=for-the-badge)](https://github.com/gadomski/stac-server-rs/actions/workflows/ci.yaml)
![Crates.io](https://img.shields.io/crates/l/stac-server?style=for-the-badge)
[![Contributor Covenant](https://img.shields.io/badge/Contributor%20Covenant-2.1-4baaaa.svg?style=for-the-badge)](./CODE_OF_CONDUCT)

A [STAC API](https://github.com/radiantearth/stac-api-spec) written in Rust.

| Crate | Description | Badges |
| ----- | ---- | --------- |
| [stac-api-backend](./stac-api-backend/README.md) | Generic backend interface for STAC APIs | [![docs.rs](https://img.shields.io/docsrs/stac-api-backend?style=flat-square)](https://docs.rs/stac-api-backend/latest/stac-api-backend/) <br> [![Crates.io](https://img.shields.io/crates/v/stac-api-backend?style=flat-square)](https://crates.io/crates/stac-api-backend) |
| [pgstac-api-backend](./pgstac-api-backend/README.md) | API backend for [pgstac](https://github.com/stac-utils/pgstac) | [![docs.rs](https://img.shields.io/docsrs/pgstac-api-backend?style=flat-square)](https://docs.rs/pgstac-api-backend/latest/pgstac_api_backend/) <br> [![Crates.io](https://img.shields.io/crates/v/pgstac-api-backend?style=flat-square)](https://crates.io/crates/pgstac-api-backend) |
| [stac-server](./stac-server/README.md) | A STAC API server in [axum](https://github.com/tokio-rs/axum) | [![docs.rs](https://img.shields.io/docsrs/stac-server?style=flat-square)](https://docs.rs/stac-server/latest/stac_server/) <br> [![Crates.io](https://img.shields.io/crates/v/stac-server?style=flat-square)](https://crates.io/crates/stac-server)
| [stac-server-cli](./stac-server-cli/README.md) | A command-line interface for [stac-server](./stac-server/README.md) | [![docs.rs](https://img.shields.io/docsrs/stac-server-cli?style=flat-square)](https://docs.rs/stac-server-cli/latest/stac_server_cli/) <br> [![Crates.io](https://img.shields.io/crates/v/stac-server-cli?style=flat-square)](https://crates.io/crates/stac-server-cli) |

## Usage

You'll need [rust](https://rustup.rs/).
Then:

```shell
cargo install stac-server-cli
```

Any collections, items, or item collections provided on the command line will be ingested into the backend on startup.
To start a memory-backed server populated with one collection and one item from the [Planetary Computer](https://planetarycomputer.microsoft.com/):

```shell
stac-server \
    https://planetarycomputer.microsoft.com/api/stac/v1/collections/3dep-seamless \
    https://planetarycomputer.microsoft.com/api/stac/v1/collections/3dep-seamless/items/n34w116-13
```

If you have a [pgstac](https://github.com/stac-utils/pgstac) database pre-populated with collections and items, you can point your server there:

```shell
stac-server --pgstac postgres://username:password@localhost/postgis
```

For more advanced setups, use a [configuration file](#configuration):

```shell
stac-server --config config.toml
```

## Configuration

The [`Config` structure](https://docs.rs/stac-server/latest/stac-server-cli/struct.Config.html) defines the configuration attributes available for your server.
This repository includes [a default configuration](./stac-server-cli/src/config.toml) that you can then customize for your use-case.

## Conformance classes

The STAC API spec uses "conformance classes" to describe the functionality of a server.
These are the supported conformance classes for each backend:

| Conformance class | Memory backend | pgstac backend |
| -- | -- | -- |
| [Core](https://github.com/radiantearth/stac-api-spec/tree/main/core) | ✅ | ✅ |
| [Features](https://github.com/radiantearth/stac-api-spec/tree/main/ogcapi-features) | ✅ | ✅ |
| [Item search](https://github.com/radiantearth/stac-api-spec/tree/main/item-search) | ❌ | ❌ |

## Validation

Conformance classes are validated with [stac-api-validator](https://github.com/stac-utils/stac-api-validator) in [CI](https://github.com/gadomski/stac-server-rs/actions/workflows/validate.yaml).
To validate yourself, you'll need to install **stac-api-validator**, preferably in a virtual enviroment:

```shell
pip install stac-api-validator
```

Then, with the memory backend:

```shell
scripts/validate
```

To validate the server with the pgstac backend, you'll need to start a pgstac server first:

```shell
docker-compose -f scripts/docker-compose.yaml up --detach
scrips/validate --pgstac
docker-compose down
```

### Conda

If you like [conda](https://docs.conda.io), you can use our provided environment to validate:

```shell
conda env create -f scripts/environment.yaml
conda activate validate
scripts/validate
```

## License

**stac-server-rs** is dual-licensed under both the MIT license and the Apache license (Version 2.0).
See [LICENSE-APACHE](./LICENSE-APACHE) and [LICENSE-MIT](./LICENSE-MIT) for details.
