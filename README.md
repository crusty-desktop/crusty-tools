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

- [X] Multiple software list files
    - [ ] Write tests, many tests ...
- [ ] Implement Alias