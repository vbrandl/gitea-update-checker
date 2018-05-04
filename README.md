# Gitea Update Checker

Check if a Gitea instance is up-to-date.

The checker requests the GitHub releases API for `go-gitea/gitea` and the `/api/v1/version` endpoint on a Gitea
instance. The version numbers are parsed, compared and the result is printed.

## Releases

For the `-musl` releases to work, one needs to set the `SSL_CERT_DIR` environment variable: `SSL_CERT_DIR=/etc/ssl/certs
./gitea-update-checker`
