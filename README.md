# DevCA (WIP)

`devca` is a command to easily generate certificates for local TLS services during development. Client applications, particularly browsers, behave differently when connecting to plaintext services (like HTTP) compared to secure services (like HTTPS). By creating certificates for your local in-development services, you can develop your services in a more similar environment to a real deployment.

## How it works

`devca` uses a vendored copy of `openssl` to generate a self-signed certificate authority (CA). It uses that CA to generate certificates for any name that you'd like. In order to get your web browser (or other application) to trust the certificates generated by `devca`, you need to configure it to trust the CA. Example instructions for Firefox are [here](./firefox-ca-install.md). Then you need to configure your in-development service to use the certificate that was signed by the CA for TLS/SSL.

## Disclaimer

`devca` is not designed to be used as anything other than a development certificate authority. It should not be used for managing a real certificate authority. All certificates generated using this tool should be kept private. Most importantly, DO NOT share the CA certificate that you install in your browser with anyone. If an attacker got hold of your development CA, they would be able to impersonate any website to you over HTTPS.

## Usage

### `new` command

`devca new` creates a new certificate for your service to use.

#### Usage

```
devca new <name>
```

#### Examples

```
$ devca new localhost
Created certificate authority private key: /home/grant/.local/share/devca/ca/key.pem
Created certificate authority certificate: /home/grant/.local/share/devca/ca/cert.pem
Created private key for "localhost": /home/grant/.local/share/devca/certs/localhost/key.pem
Created certificate for "localhost": /home/grant/.local/share/devca/certs/localhost/cert.pem
```

The above is an example of what to expect when creating a certificate for the first time. `devca` will notice that it hasn't created a CA yet and generate it. It will then use the new CA to sign a new certificate for the name requested.

Subsequent runs will not require generating the CA, but will reuse the one it has already created.

```
$ devca new mydevwebsite.local
Created private key for "mydevwebsite.local": /home/grant/.local/share/devca/certs/mydevwebsite.local/key.pem
Created certificate for "mydevwebsite.local": /home/grant/.local/share/devca/certs/mydevwebsite.local/cert.pem
```

Requesting a cert for a name that already has a cert will require confirming your action, and then it will overwrite the existing cert with a new one.

```
$ devca new localhost
**** A certificate for "localhost" already exists. Would you like to overwrite it? y/N: y
Created private key for "localhost": /home/grant/.local/share/devca/certs/localhost/key.pem
Created certificate for "localhost": /home/grant/.local/share/devca/certs/localhost/cert.pem
```

### `ls` command

Coming soon...

### `pwd` command

Coming soon...

### `ln` command?

Coming soon...

### `rm` command

Coming soon...

### `regen` command

Coming soon...
