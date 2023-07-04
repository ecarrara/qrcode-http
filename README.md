# qrcode-http

Serve QR Code images over HTTP. Supports PNG, SVG and Unicode (text) formats.

## Usage

Start a server.

```
qrcode-http --addr 0.0.0.0:8010
```

### PNG support

Make a request to `http://localhost:8000/?code=contents` to generate a QRCode in PNG format (default). 

```
curl 'http://localhost:8000/?code=the+lunatic+is+on+the+grass&format=png' > example.png
open example.png
```

### Unicode support

You can also generate a text/unicode output.

```
$ curl 'http://localhost:8000/?code=the+lunatic+is+on+the+grass&format=txt'
▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄
█ ▄▄▄▄▄ █▄█▄▀██ ▄▀█▄█ █ ▄▄▄▄▄ █
█ █   █ █ ▀▄▄█▄▄▀ ▄█ ██ █   █ █
█ █▄▄▄█ █▀▀█▀▄ ▀ ▀  ▀██ █▄▄▄█ █
█▄▄▄▄▄▄▄█ █▄▀▄█▄█▄█▄█▄█▄▄▄▄▄▄▄█
█▀ █ █▄▄▄▄██▀ █ ▄▄ █▄   ▄█ ▄▀ █
██ ▀▄█▄▄▄  █▄▀█▄ ▀█▄▄▀▄██▄▄  ██
██   ▀▄▄▄  ▀█▄▀█▀  █▀  ▄▄█▄▀ ▄█
█ ▀ ▄▄█▄  ▄▀ ▀▀ ▄▄  ▀▀▄ ▄ ██▀ █
██▄█ ▀▀▄▄██▀▀ ▄▀ ▀▀ ▄█▄▀██▄▀▄ █
█▄██▄ ▀▄█ ██▄  ▄▄▄▀ ██ █▄▄▀▄▄▄█
█▄█▄▄▄█▄▄ ▄ ▀▀██▄ █▀█ ▄▄▄ █▄▀▀█
█ ▄▄▄▄▄ █ █▄▄▄█▄▀█ ██ █▄█ ▀ ▄▀█
█ █   █ █▀▄   ▄▀▀▀▀█▀▄▄   █▀▄▀█
█ █▄▄▄█ █▀█▀▄ ▀█▀█▀ ▀▀▀█▀▄██▀▄█
█▄▄▄▄▄▄▄██▄██▄█▄██▄▄█▄█▄█▄▄█▄██
```


### SVG support


```
$ curl 'http://localhost:8000/?code=the+lunatic+is+on+the+grass&format=svg' > example.svg
open example.svg
```

## Built With

This software uses the following open source projects:

* [axum](https://github.com/tokio-rs/axum)
* [fast_qr](https://github.com/erwanvivien/fast_qr) 