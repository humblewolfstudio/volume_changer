# Volume-Changer

TCP Server to modify the volume via tcp message

## Compatibility

- [x] MacOS
- [ ] Windows

## Usage

Run local project in rust:

You can pass a 4 digits pass so you can control who can use the service. If not, it'll generate a random code and print it in the console.

Example:
```console
foo@bar:~$ cargo run UF4R
```

Build project to target folder:

```console
foo@bar:~$ cargo build
```

## Volume Control Service

MacOS service developed by [kmunoz99](https://github.com/kmunoz99)

_Missing Link_

## TODO

Windows compatibility (Maybe implement sound management with this?? https://github.com/RustAudio/cpal)
