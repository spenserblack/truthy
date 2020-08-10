# Changelog

## [Unreleased](https://github.com/spenserblack/truthy-rs/compare/v1.1.0...HEAD)
### Changed
- `impl Truthy for str` to `&str`
  - Allows `String` to inherit implementation from `Deref<Target=str>`

## [1.1.0]
### Added
- `and-or` feature
- `truthy!` macro

## 1.0.0 :tada:
Initial version

[1.1.0]: https://github.com/spenserblack/truthy-rs/compare/v1.0.0...v1.1.0
