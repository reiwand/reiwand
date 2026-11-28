# HTTPS

`http://` is only valid for the server's own computer. A server on another computer needs HTTPS.
There are two ways: the server's own certificate on a home network, or a reverse proxy with a
domain of your own.

## On a home network, with the server's own certificate

On a home network without a domain, the server can serve HTTPS itself with a certificate of its
own, with the address the administrator gives by hand:

```sh
REIWAND_TLS=self-signed REIWAND_LISTEN=0.0.0.0:8443 \
  REIWAND_PUBLIC_URL=https://192.168.1.10:8443 reiwand-server serve
reiwand-server fingerprint                    # the SHA-256 fingerprint of the certificate, also in the log
reiwand-ctl remote add https://192.168.1.10:8443/r/main --trust-certificate <fingerprint>
```

The certificate is created on the first start in `/data/tls` and kept.

To add the repository the first time:

1. The app and `remote add` say that the server uses its own certificate and show its
   fingerprint.
2. Compare it with that of `reiwand-server fingerprint`.
3. Confirm it ("Trust and add", or `--trust-certificate`).

The certificate is pinned only for that server. If it later presents another, a warning is given
and nothing is applied.

With a home-network server everything works the same as with a public one, without leaving the
network.

## With a domain, behind a reverse proxy

With a domain of your own, any reverse proxy with TLS works:

- The server listens on `REIWAND_BIND` (`127.0.0.1:8080` by default).
- The proxy terminates HTTPS and forwards the requests.
- `REIWAND_PUBLIC_URL` carries the public address with `https://`.

### Client addresses

The server takes the client address from `X-Forwarded-For` when the proxy runs on the same
computer (also with the image of `compose.yaml`). A proxy in another container or on another
machine is named in `REIWAND_TRUSTED_PROXIES`.

A computer of the home network that talks directly to the server cannot pass itself off as another
address.

### Nginx example

[`deploy/nginx/reiwand.conf`](../../../deploy/nginx/reiwand.conf) is an example for Nginx:

```sh
sudo cp deploy/nginx/reiwand.conf /etc/nginx/conf.d/wallpapers.conf   # change the name and the certificates
sudo nginx -t && sudo systemctl reload nginx
```

The example:

- Serves `wallpapers.example.org` on 443 (and redirects 80).
- Has the certificate paths as placeholders.
- Sets `client_max_body_size 256m`, the default maximum package size
  (`REIWAND_PACKAGE_MAX_MIB`).
- Uses HTTP/1.1 with `Connection ""` toward the server, without buffering uploads on disk.

> **Note:** If you change `REIWAND_PACKAGE_MAX_MIB`, raise `client_max_body_size` in step.

The server uses no WebSockets or long connections.
