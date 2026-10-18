# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Security

- Refreshed `Cargo.lock` (`cargo update -p url`): `url` 2.5.2 → 2.5.8 moves `idna` 0.5.0 → 1.1.0, which fixes RUSTSEC-2024-0421 (`idna` accepted Punycode labels that decode to no non-ASCII characters)

## [0.3.0] - 2026-02-13

### Changed

- Fixed `py_svg_hush.pyi` signature for `filter_svg` to match runtime default (`keep_data_url_mime_types=None`)
- Updated `py_svg_hush.pyi` stub: modernized type syntax to PEP 585/604 (`dict`, `list`, `X | None`), removed `typing` imports
- Relaxed `build-system` requirement to `maturin>=1.0,<2.0` (instead of pinning a single patch version)
- Removed explicit `pyo3/extension-module` feature from `tool.maturin` now that modern `maturin` handles extension-module automatically
- Upgraded `pyo3` → 0.28.0
- Upgraded `maturin` → 1.11.5
- Raised minimum Python version to 3.10 (dropped 3.9)
- Regenerated GitHub CI config (`maturin generate-ci`)
  - Updated GitHub Actions: `checkout` v6, `setup-python` v6, `upload-artifact` v6, `download-artifact` v7, `attest-build-provenance` v3, `run-on-arch-action` v3
  - Replaced unmaintained `addnab/docker-run-action@v3` with direct `docker run` for musllinux x86_64 pytest

## [0.2.0] - 2025-03-15

### Changed

- Upgraded `maturin` → 1.8.3
- Upgraded `pyo3` → 0.24.0
- Upgraded `svg-hush` → 0.9.5
- Upgraded `data-url` → 0.3.1
- Upgraded `ruff` → 0.11.0

## [0.1.2] - 2024-09-24

### Added

- `pip` dependency for `maturin develop`

### Changed

- Upgraded `maturin` → 1.7.2
- Upgraded `ruff` → 0.6.7
- Upgraded `pytest` → 8.3.3
- Upgraded `svg-hush` → 0.9.4
- Regenerated GitHub CI config

### Removed

- `black` dependency

## [0.1.0] - 2023-03-31

### Added

- `filter_svg` function
