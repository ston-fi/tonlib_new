# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.0.15](https://github.com/Sild/ton_lib_rs/compare/ton_lib-v0.0.14...ton_lib-v0.0.15) - 2025-05-01

### Other

- don't run coverage on MR

## [0.0.14](https://github.com/Sild/ton_lib_rs/compare/ton_lib-v0.0.13...ton_lib-v0.0.14) - 2025-04-30

### Other

- Merge pull request #9 from Sild/into-coint-impl

## [0.0.13](https://github.com/Sild/ton_lib_rs/compare/ton_lib-v0.0.12...ton_lib-v0.0.13) - 2025-04-27

### Other

- Merge pull request #7 from Sild/add-libs-dict

## [0.0.12](https://github.com/Sild/ton_lib_rs/compare/ton_lib-v0.0.11...ton_lib-v0.0.12) - 2025-04-26

### Other

- add LiteClient, TonlibClient (TLClient)

## [0.0.10](https://github.com/Sild/libs_rs/compare/ton_lib-v0.0.9...ton_lib-v0.0.10) - 2025-04-20

### Other

- Build dict bench ([#51](https://github.com/Sild/libs_rs/pull/51))

## [0.0.9](https://github.com/Sild/libs_rs/compare/ton_lib-v0.0.8...ton_lib-v0.0.9) - 2025-04-20

### Other

- add TLBRef alias for derive
- use bits_len as alias for adapter=ConstLen

## [0.0.8](https://github.com/Sild/libs_rs/compare/ton_lib-v0.0.7...ton_lib-v0.0.8) - 2025-04-19

### Other

- add Coins::zero() constructor
- add TonAddress converters + TLBType, fix TLBDerive for Enums

## [0.0.7](https://github.com/Sild/libs_rs/compare/ton_lib-v0.0.6...ton_lib-v0.0.7) - 2025-04-18

### Other

- small fixes after using in external project

## [0.0.6](https://github.com/Sild/libs_rs/compare/ton_lib-v0.0.5...ton_lib-v0.0.6) - 2025-04-18

### Fixed

- fix flack test

### Other

- impl Account TLB
- generalize adapters
- support adapter="TLBRef"
- Update adapter interfaces ([#46](https://github.com/Sild/libs_rs/pull/46))

## [0.0.5](https://github.com/Sild/libs_rs/compare/ton_lib-v0.0.4...ton_lib-v0.0.5) - 2025-04-14

### Other

- Support dict ([#45](https://github.com/Sild/libs_rs/pull/45))
- cleanup naming, update imports, add TLBRef, TonCellRef now behave like TonCell in TLB context
- cleanup verify_prefix

## [0.0.4](https://github.com/Sild/libs_rs/compare/ton_lib-v0.0.3...ton_lib-v0.0.4) - 2025-04-05

### Fixed

- fix formatting

### Other

- add TLB prefix to tlb entities, add tests for dyn_len types ([#43](https://github.com/Sild/libs_rs/pull/43))
- Update export ([#42](https://github.com/Sild/libs_rs/pull/42))
- Add msg address ([#41](https://github.com/Sild/libs_rs/pull/41))

## [0.0.3](https://github.com/Sild/libs_rs/compare/ton_lib-v0.0.2...ton_lib-v0.0.3) - 2025-04-01

### Other

- ton_lib_proc_macro, TLBDerive ([#39](https://github.com/Sild/libs_rs/pull/39))
- only TonCell left ([#38](https://github.com/Sild/libs_rs/pull/38))
- implement write/read big_num ([#36](https://github.com/Sild/libs_rs/pull/36))

## [0.0.2](https://github.com/Sild/libs_rs/compare/ton_lib-v0.0.1...ton_lib-v0.0.2) - 2025-03-07

### Other

- init tonlib ([#35](https://github.com/Sild/libs_rs/pull/35))
- release v0.3.1 ([#33](https://github.com/Sild/libs_rs/pull/33))
