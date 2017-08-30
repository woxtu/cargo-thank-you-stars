# cargo-thank-you-stars

Give your dependencies stars on GitHub!
This is cargo subcommand implementation of [teppeis/thank-you-stars](https://github.com/teppeis/thank-you-stars).

## Installation

`cargo-thank-you-stars` can be installed with `cargo install`:

```console
$ cargo install --git https://github.com/woxtu/cargo-thank-you-stars
```

Save your GitHub personal access token:

1. Open https://github.com/settings/tokens and click "Generate new token"
2. Input desc, select only "public_repo" as scope and click "Generate token"
3. Copy the token and save as `~/.thank-you-stars.json`

```json
{
    "token": "YOUR_TOKEN"
}
```

## Usage

Run in your project root directory after `cargo install`.

```console
$ cargo thank-you-stars
```

## License

Licensed under the MIT License.
