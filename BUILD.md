The project is divided into two parts:

- **core_lib:** This is a Rust library that encompasses all the logic necessary for discovering, connecting to, and transferring files to QuickShare-compatible clients.
- **app:** Both GUI and NO-GUI version of the client.

How to build
--------------------------

### core_lib

Building the core_lib is straightforward because it is a basic Rust project. Simply use `cargo build` to compile it.

### app/gui

The GUI version is developed as a Tauri application. For package management, pnpm is recommended (though npm and others may also work, pnpm is preferred for this project).

(all commands are run inside the `app/gui` folder)

First, install the necessary dependencies:

```
pnpm install
```

- To run the debug version:

```
pnpm dev
```

- To build a release package (.deb & .AppImage):

```
pnpm build
```

For more detailed information on building the GUI and understanding any potential limitations, it’s advised to consult the [Tauri documentation](https://tauri.app/v1/guides/building/linux).