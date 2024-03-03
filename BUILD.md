The project is divided into two parts:

- **core_lib:** This is a Rust library that encompasses all the logic necessary for discovering, connecting to, and transferring files to QuickShare-compatible clients.
- **frontend:** A Tauri application that utilizes core_lib to handle incoming requests and initiate outgoing ones.

How to build
--------------------------

### core_lib

Building the core_lib is straightforward because it is a basic Rust project. Simply use `cargo build` to compile it.

### frontend

The frontend is developed as a Tauri application. For package management, pnpm is recommended (though npm and others may also work, pnpm is preferred for this project).

(all commands are run inside the `frontend` folder)

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

For more detailed information on building the frontend and understanding any potential limitations, it’s advised to consult the [Tauri documentation](https://tauri.app/v1/guides/building/linux).