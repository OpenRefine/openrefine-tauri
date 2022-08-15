# openrefine-tauri

First, please follow all instructions to setup your dev environment as can be found here:

> https://tauri.app/v1/guides/getting-started/prerequisites

Our preference is to use cargo to run tauri, instead of yarn/npm, just because it is a bit faster.

Once your rust toolchain is setup, please run:
```
cargo install tauri-cli --version ^1.0.5
```

Then to install nodejs dependencies:
```
> yarn
```

And finally to install rust dependencies and compile:
```
> cargo tauri dev
```

To see the development process, and when you are ready to bundle the binary:

```
> cargo tauri build
```

If at any time you want to confirm that your dependencies are up to date, run:

```
> cargo tauri info
```

And, if for some reason you need to debug the bundled binary, run:
```
> cargo tauri build --debug
```

Once you are done building, the binary executable will be found in the `src-tauri/target/release/bundle` folder.
