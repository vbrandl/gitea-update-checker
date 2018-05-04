# Gitea Update Checker

[![Travis Build
Status](https://travis-ci.org/vbrandl/gitea-update-checker.svg?branch=master)](https://travis-ci.org/vbrandl/gitea-update-checker)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](https://github.com/vbrandl/gitea-update-checker/blob/master/LICENSE-MIT)
[![License](https://img.shields.io/badge/license-Apache-green.svg)](https://github.com/vbrandl/gitea-update-checker/blob/master/LICENSE-APACHE)

Check if a Gitea instance is up-to-date.

The checker requests the GitHub releases API for `go-gitea/gitea` and the `/api/v1/version` endpoint on a Gitea
instance. The version numbers are parsed, compared and the result is printed.

## Releases

For the `-musl` releases to work, one needs to set the `SSL_CERT_DIR` environment variable: `SSL_CERT_DIR=/etc/ssl/certs
./gitea-update-checker`

## License

gitea-update-checker is licensed under either of the following, at your option:

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT License ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
