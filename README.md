# embedded-http

Rust library for creating HTTP/1.1 requests on embedded systems

## Before pushing new versions:

Check that the different features compile and work as expected:

```bash
cargo hack check --feature-powerset
```

```bash
cargo hack test --feature-powerset --skip defmt
```
