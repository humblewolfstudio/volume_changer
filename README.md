# Volume-Changer

TCP Server to modify the volume via tcp message

## Compatibility

### Compatible OS

- [x] MacOS
- [ ] Windows


### Compatible Software

|Software|Play/Pause Media |Next Media |Prev Media|
|Spotify| [x] | [x] | [x] |
|VLC | [ ] | [ ] | [ ] |
|IINA | [ ] | [ ] | [ ] |
|QuickTime Player | [ ] | [ ] | [ ] |


## Usage

Run local project in rust:

You can pass a 4 digits code so you can control who can use the service. If not, it'll generate a random code and print it in the console.

Example with custom code:
```console
foo@bar:~$ cargo run UF4R
```

Example random code code:
```console
foo@bar:~$ cargo run
```

Build project to target folder:

```console
foo@bar:~$ cargo build
```

## Volume Control Service

[MacOS and iOS](https://gitlab.verde-loro.com/byteremote/byteremote) app developed by [kmunoz99](https://github.com/kmunoz99)