# pngme

## Original tutorial

https://jrdngr.github.io/pngme_book/introduction.html


## Running

Compile
```
cargo build
```

Help
```
pngme -h 
pngme encode -h
pngme decode -h
pngme remove -h
```

Encode an original message into a PNG image
```
pngme encode -f ./dice.png -m "Hello Rust"
```

Decode and get the original message

```
pngme decode -f ./dice.png
```

Remove the encoded message

```
pngme remove -f ./dice.png
```