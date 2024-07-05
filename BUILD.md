The project is divided into two parts:

- **core_lib:** This is a Rust library that encompasses all the logic necessary for discovering, connecting to, and transferring files to QuickShare-compatible clients.
- **app/legacy:** A Tauri application that utilizes core_lib to handle incoming requests and initiate outgoing ones.

How to build
--------------------------

### core_lib

Building the core_lib is straightforward because it is a basic Rust project. Simply use `cargo build` to compile it.

### app/main

The app/main is developed as a Tauri application. For package management, pnpm is recommended (though npm and others may also work, pnpm is preferred for this project).

(all commands are run inside the `app/main` folder)

First, install the necessary dependencies:

```
pnpm install
```

- To run the debug version:

```
pnpm dev
```

- To build a release package (.deb & .AppImage & .rpm & .dmg (only on macos)):

```
pnpm build
```

For more detailed information on building the app/legacy and understanding any potential limitations, it’s advised to consult the [Tauri documentation](https://v2.tauri.app/start).

### app/legacy

Everything is the same as the app/main one, except the output of the build :)

- To build a release package (.deb & .AppImage):

```
pnpm build
```

For more detailed information on building the app/legacy and understanding any potential limitations, it’s advised to consult the [Tauri documentation](https://tauri.app/v1/guides/building/linux).
