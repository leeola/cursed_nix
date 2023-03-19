
# Cursed Nix (unstable)

An attempt at writing the Nix Language via Rust Language. Imperative Nix config generation. Super cursed.

More generally, this repo is a series of research focused crates to answer the following questions:

1. Can i write Nix in Rust?
2. How badly will i regret doing this?
3. In what specific ways is this a bad idea? What specifically will i regret about doing this? 
4. In the process of writing some cursed Nix "replacement", will i grow to appreciate and love
   Nix for what it is?

## FAQ

### Isn't this a bad idea?

Most definitely. I mean, i named the project Cursed Nix. See above.

### Do you feel bad for doing this?

Not yet, but i'm sure i will.

### Why would you do this?

Because i like Rust. Because i like my editor to have a feature rich LSP,
and Nix is really difficult to write a good LSP for. Because my UX in Rust is great and
my Rust LSP is great.

### What about Nickel?

Nickel is really cool and when i can write Nickel instead of Nix i just may.
Though there's a good chance i'd still prefer to write it in Rust, in a perfect world.

### But Rust isn't pure and functional like Nix! 

Firstly, i said it was cursed. Secondaly, yup. This project would allow you to write
Nix that compiles differently every time. You could programatically create all sorts of bizarre
configuration. You can even work around features or tooling you don't like or understand in Nix by
expanding them out verbosely in Rust. As if this crate is one big macro for Nix.

This project is a Choose Your Own Adventure for Nix. Do with that what you will.
