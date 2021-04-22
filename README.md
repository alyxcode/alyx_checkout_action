# Alyx `checkout` GitHub Action

[![CI](https://img.shields.io/github/workflow/status/alyxcode/alyx_checkout_action/default/default?color=5e81ac&label=CI&logo=github&style=for-the-badge)](https://github.com/alyxcode/alyx_checkout_action/actions)

This action checkout the git repository for use in [actions](https://github.com/features/actions).

## Usage

```yaml
on:
  push:
    branches: [ default ]
jobs:
  checkout:
    strategy:
      matrix:
        os: [ubuntu-latest, macos-latest, windows-latest]
    runs-on: ${{ matrix.os }}
    steps:
      - uses: alyxcode/alyx_checkout_action@default
```

## License

[![License](https://img.shields.io/badge/license-mit-81a1c1?style=for-the-badge)](LICENSE)