based on https://github.com/TheBevyFlock/bevy_quickstart

# Bevy game jam 05
## todo
- could listen to OnAdd for Observer and inject a Name component on the entity

<details>
  <summary>todo</summary>
- Simplify / review `docs/` files.
- read doc/design
- consider .cargo/config_fast_builds
- try [3rd-party tools](./docs/tooling.md)
-- avian
-- leafwing-input-manager
-- asset_loader, common_assets, iyes_progress
</details>

## src / build notes
regex to find title etc ```jam.bevy.05.cringe```

## Run your game

Running your game locally is very simple:

- Use `cargo run` to run a native dev build.
- Use [`trunk serve`](https://trunkrs.dev/) to run a web dev build.

If you're using [VS Code](https://code.visualstudio.com/), this template comes with a [`.vscode/tasks.json`](./.vscode/tasks.json) file.

<details>
  <summary>Run release builds</summary>

- Use `cargo run --profile release-native --no-default-features` to run a native release build.
- Use `trunk serve --release --no-default-features` to run a web release build.

</details>

<details>
    <summary>(Optional) Improve your compile times</summary>

[`.cargo/config_fast_builds.toml`](./.cargo/config_fast_builds.toml) contains documentation on how to set up your environment to improve compile times.
After you've fiddled with it, rename it to `.cargo/config.toml` to enable it.

</details>

## Release your game

This template uses [GitHub workflows](https://docs.github.com/en/actions/using-workflows) to run tests and build releases.
See [Workflows](./docs/workflows.md) for more information.

## Known Issues

There are some known issues in Bevy that require some arcane workarounds.
To keep this template simple, we have opted not to include those workarounds.
You can read about them in the [Known Issues](./docs/known-issues.md) document.

## License

The source code in this repository is licensed under any of the following at your option:

- [CC0-1.0 License](./LICENSE-CC0-1.0.txt)
- [MIT License](./LICENSE-MIT.txt)
- [Apache License, Version 2.0](./LICENSE-Apache-2.0.txt)

We hold no patent rights to anything presented in this repository.

## Credits

The [assets](./assets) in this repository are all 3rd-party. See the [credits screen](./src/screen/credits.rs) for more information.
