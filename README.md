# Ilujo

## Install

### System requirements

- A shell like _bash_ or _zsh_
- _Rust compiler_. More info [here](https://www.rust-lang.org/tools/install)

### Steps

- Clone the repository.
- Enter the repo directory and run the install script:

```sh
./install # or: bash install
```

- To have access to the executable, add the `$HOME/bin`to your `$PATH` env variable. For instance, add a like at the end of your _.zshrc_ file:

```sh
export $PATH=$PATH:$HOME/bin
```

### Outcome

- A new directory `$HOME/.ilujo` will be created.
- A link will be created to the `$HOME/bin`directory.

## Usage

- In the desired directory:

```sh
ilujo ui MyNewComponent
```

- If you want to customize the templates, go to the application directory:

```sh
ilujo open-dir
```

- To print the help message:

```sh
ilujo help
```

- In case you want to see the debug mode, pass a `ILUJO_DEBUG_MODE=true` env variable:

```sh
ILUJO_DEBUG_MODE=true ilujo ui HelloWorld
```

## Notes

_ilujo_ is the word for _toolbox_ in esperanto
