## TODO List

- [ ] Big refactoring

```toml
[flatpak.lala]
[pkg.lalaland]
[pkg.lalaland2]
[rust.zapa]
[rust.zapa2]
```

Bonus! Flexibility each installer can have different options

```rust
trait Package {
    fn exists() -> bool;
    fn install() -> Result();
}
```

- How to handle dependencies:

```toml
deps = ["system::package", "mc", "rust:eza", "flatpack:prog.cansas.com"]
```

- [X] Make source optional. If missing use the name
    - Write tests, many tests ...
- [X] Make category optional
- [ ] Skip optional packages
    - [ ] add -o --optional --no-optional
- [X] Multiple software list files
    - [ ] If one is optional and other is not make it not optional
    - [ ] Write tests, many tests ...
- [ ] Implement Dry Run
- [ ] Implement Alias