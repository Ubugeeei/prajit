# prajit
## Usage
### inputs command line arguments
```sh
$ cargo run "1 + 2 * 3"
# 7

$ cargo run "1 + 2 * 3" "1 + 2 * 3" "1 + 2 * 3"
# 7 7 7
```

### interactive mode
```sh
$ cargo run -- -i
# or
$ cargo run -- --interactive

> 1 + 1
# 2
> 5 * 2
# 10
> exit
# bye.
```