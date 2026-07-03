# rtotp

Rust one-Time Password - a small terminal TOTP (RFC 6238) client.

Stores secrets in `~/.rtotplist` (JSON, `0600`) and prints time-based codes.
A Rust take on [jonnung/gtp](https://github.com/jonnung/gtp), with a private
config file and hidden secret entry.

## Demo

![rtotp demo](assets/demo.gif)

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
```

`rtotp 1` is shorthand for `rtotp code 1`.

## Notes

- The secret is stored locally in plaintext at `~/.rtotplist` (file mode `0600`).
  Treat the machine as the trust boundary.
- Codes are standard TOTP: SHA1, 6 digits, 30s period - compatible with most
  authenticators and OTP-issuing apps.

## License

MIT
