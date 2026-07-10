# rtotp

Rust one-Time Password - a small terminal TOTP (RFC 6238) client.

`rtotp` stores TOTP secrets locally and prints current 6-digit codes on demand.
It can also render an `otpauth://` QR code in the terminal so the same entry can
be imported into Google Authenticator or another authenticator app.

## Demo

![rtotp demo](assets/rtotp-demo.gif)

## Install

Download a prebuilt binary from the [releases page](https://github.com/goodhee/rtotp/releases),
or build from source:

```sh
cargo install --git https://github.com/goodhee/rtotp
# or, from a checkout:
cargo build --release   # -> target/release/rtotp
```

## Usage

```
rtotp <command>

  <n>       Show the current 6-digit code for entry n (see `list`)
  code      Show the current 6-digit code for entry n
  qr        Show a terminal QR code for adding an entry to an authenticator app
  add       Add a new OTP secret (Issuer, Account Name, Secret)
  list      List registered secrets
  remove    Remove an entry (prompts if index omitted)
  clear     Remove all entries
```

Example:

```sh
rtotp add            # Issuer: keep, Account Name: owner, Secret: <base32>
rtotp list           # {1} keep:owner
rtotp 1              # 482910
rtotp qr 1           # scan with Google Authenticator or another OTP app
```

`rtotp 1` is shorthand for `rtotp code 1`.

## Authenticator app import

To copy an existing `rtotp` entry to a phone:

```sh
rtotp list
rtotp qr 1
```

Open Google Authenticator or another TOTP app, choose the QR scan flow, and scan
the code shown in the terminal.

## Notes

- The secret is stored locally in plaintext at `~/.rtotplist` (file mode `0600`).
  Treat the machine as the trust boundary.
- `rtotp qr <n>` displays an `otpauth://` QR code containing the secret. Anyone
  who can scan or capture that QR code can clone the OTP entry.
- Codes are standard TOTP: SHA1, 6 digits, 30s period - compatible with most
  authenticators and OTP-issuing apps.

## License

MIT
