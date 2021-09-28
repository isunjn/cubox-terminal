# Cubox-Terminal

Take [cubox](https://cubox.pro) memo in your terminal.

## Install

```sh
cargo install cubox
```

The command will be installed as `cu`

## Usage

First, set up your API key, which you get in your cubox settings (Premium required).

```sh
cu -k xxxxxxxxxxx
```

Then, take memo like this:

```sh
cu some memo text @folder ^title ::tag1 ::tag2 %description
```

Only memo text is necessary.

Optional: `@` for folder, `^` for title, `::` for a tag, `%` for description

Example:

```sh
cu Nothing is ture, Everything is permitted @assassin ^creed ::game ::assassin
```

You can also bookmark a url via cu:

```sh
cu -l https://example.com
```

folder, title, tag, description works optional too.

More usage info, type `cu --help`

## Todo

- [ ] Test
- [ ] Binary distribution
- [ ] Support editing using vim or $EDITOR

## Contributing

Issue and PR are welcomed.

## License

[MIT](LICENSE)
