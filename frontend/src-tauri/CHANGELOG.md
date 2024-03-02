# Changelog

## [0.3.0](https://github.com/Martichou/rquickshare/compare/rquickshare-v0.2.2...rquickshare-v0.3.0) (2024-03-02)


### Features

* don't use hardcoded key anymore ([c63f223](https://github.com/Martichou/rquickshare/commit/c63f223ca855e4bb9013a397d47ccaa12ea0d61d))

## [0.2.2](https://github.com/Martichou/rquickshare/compare/rquickshare-v0.2.0...rquickshare-v0.2.2) (2024-03-01)

### Miscellaneous Chores

* release 0.2.2 ([912fe2c](https://github.com/Martichou/rquickshare/commit/912fe2c7eb5c2d19813147f821ff3c97ddaf7c75))

## [0.2.0](https://github.com/Martichou/rquickshare/compare/rquickshare-v0.1.0...rquickshare-v0.2.0) (2024-03-01)


### Features

* add disable/enable start on boot ([c74b3c5](https://github.com/Martichou/rquickshare/commit/c74b3c535e6103d3054847f7cd8f311d26414872))
* real hostname in UI ([42d6167](https://github.com/Martichou/rquickshare/commit/42d6167da3f2f4cad11da595d74174752cec12ba))


### Bug Fixes

* notifications blocked the thread ([7987f0b](https://github.com/Martichou/rquickshare/commit/7987f0bf870b50fabd7301e11361930ca5990de6))

## 0.1.0 (2024-02-29)


### ⚠ BREAKING CHANGES

* add frontend(tauri) & move current to core_lib

### Features

* add frontend(tauri) & move current to core_lib ([d989683](https://github.com/Martichou/rquickshare/commit/d9896837d9c00c687deb66663cd9a7ed264574ac))
* autostart at boot ([e904683](https://github.com/Martichou/rquickshare/commit/e90468311f10f95ed01ff72b98977d533dc454ee))
* export common structs to TS ([c2072b5](https://github.com/Martichou/rquickshare/commit/c2072b5260938317128c022fe8ac9ac687ea8eaf))
* front-backend communication channel ([45d9d7c](https://github.com/Martichou/rquickshare/commit/45d9d7c61761e40114a6e96b4f6bc6069fc1487f))
* improve UI & handling of requests ([4bd3e83](https://github.com/Martichou/rquickshare/commit/4bd3e832cca133dbe0f6193ffcd9b0b7be42cff2))
* move tauri to V2 & BLEA behind feature ([1187b44](https://github.com/Martichou/rquickshare/commit/1187b447336ba0446264c53fb1e86f61d3e45c35))
* prepare tauri to send cmd ([fc2a28a](https://github.com/Martichou/rquickshare/commit/fc2a28a493ada7e9ffe1611bdcb960a727ea1764))
* release process ([b989b05](https://github.com/Martichou/rquickshare/commit/b989b05e6151d618d77b8e6379518eb55f9c1b1a))
* sending file working (and wip UI) ([53890f0](https://github.com/Martichou/rquickshare/commit/53890f08dde4261f1775f925ee6a3084fbe76eae))
* somewhat working UI channel with Rust ([6452132](https://github.com/Martichou/rquickshare/commit/6452132c976d0f02574855953a2f1b3431a5c28d))
* **ui:** tray allow to open app ([748cbbb](https://github.com/Martichou/rquickshare/commit/748cbbbccf165b4429c49fd5ee88f4e86405c11a))
* **wip:** outbound handle paired_key_result ([56e32c1](https://github.com/Martichou/rquickshare/commit/56e32c15a0fcf71eb821dca70960d9a64539a4a7))
* **wip:** try to send BLE advertisment when sending ([d8ae730](https://github.com/Martichou/rquickshare/commit/d8ae730ed09df3d28b5870e63c8a42423780d310))


### Bug Fixes

* linting ([428e141](https://github.com/Martichou/rquickshare/commit/428e141895ae687bd6e8befe30a2bfebe761dfb4))
* use vendored tls ([8c5b954](https://github.com/Martichou/rquickshare/commit/8c5b954645c996a7d2fdc2457ae257cd88cab464))
* wrong path for dep ([72ef7bc](https://github.com/Martichou/rquickshare/commit/72ef7bc26ba7c1c1ad017a232e9f4a3bd25fe8c5))

## [2.0.0](https://github.com/Martichou/rquickshare/compare/rquickshare-v1.0.2...rquickshare-v2.0.0) (2024-02-29)


### ⚠ BREAKING CHANGES

* add frontend(tauri) & move current to core_lib

### Features

* add frontend(tauri) & move current to core_lib ([d989683](https://github.com/Martichou/rquickshare/commit/d9896837d9c00c687deb66663cd9a7ed264574ac))
* autostart at boot ([e904683](https://github.com/Martichou/rquickshare/commit/e90468311f10f95ed01ff72b98977d533dc454ee))
* export common structs to TS ([c2072b5](https://github.com/Martichou/rquickshare/commit/c2072b5260938317128c022fe8ac9ac687ea8eaf))
* front-backend communication channel ([45d9d7c](https://github.com/Martichou/rquickshare/commit/45d9d7c61761e40114a6e96b4f6bc6069fc1487f))
* improve UI & handling of requests ([4bd3e83](https://github.com/Martichou/rquickshare/commit/4bd3e832cca133dbe0f6193ffcd9b0b7be42cff2))
* move tauri to V2 & BLEA behind feature ([1187b44](https://github.com/Martichou/rquickshare/commit/1187b447336ba0446264c53fb1e86f61d3e45c35))
* prepare tauri to send cmd ([fc2a28a](https://github.com/Martichou/rquickshare/commit/fc2a28a493ada7e9ffe1611bdcb960a727ea1764))
* release process ([ecd02d3](https://github.com/Martichou/rquickshare/commit/ecd02d364b5a54dd8b2c196091e0f200aff2c03d))
* sending file working (and wip UI) ([53890f0](https://github.com/Martichou/rquickshare/commit/53890f08dde4261f1775f925ee6a3084fbe76eae))
* somewhat working UI channel with Rust ([6452132](https://github.com/Martichou/rquickshare/commit/6452132c976d0f02574855953a2f1b3431a5c28d))
* **ui:** tray allow to open app ([748cbbb](https://github.com/Martichou/rquickshare/commit/748cbbbccf165b4429c49fd5ee88f4e86405c11a))
* **wip:** outbound handle paired_key_result ([56e32c1](https://github.com/Martichou/rquickshare/commit/56e32c15a0fcf71eb821dca70960d9a64539a4a7))
* **wip:** try to send BLE advertisment when sending ([d8ae730](https://github.com/Martichou/rquickshare/commit/d8ae730ed09df3d28b5870e63c8a42423780d310))


### Bug Fixes

* linting ([428e141](https://github.com/Martichou/rquickshare/commit/428e141895ae687bd6e8befe30a2bfebe761dfb4))
* use vendored tls ([8c5b954](https://github.com/Martichou/rquickshare/commit/8c5b954645c996a7d2fdc2457ae257cd88cab464))
* wrong path for dep ([72ef7bc](https://github.com/Martichou/rquickshare/commit/72ef7bc26ba7c1c1ad017a232e9f4a3bd25fe8c5))

## [1.0.2](https://github.com/Martichou/rquickshare/compare/rquickshare-v1.0.1...rquickshare-v1.0.2) (2024-02-29)


### Bug Fixes

* improve release process ([f08f42b](https://github.com/Martichou/rquickshare/commit/f08f42b7ab33e9bb35e7f5afa556997269f872c1))
* rename pkg for prettier name ([122d38a](https://github.com/Martichou/rquickshare/commit/122d38a2dff944e0d3e2aa5c90138cb586467d0a))

## [1.0.1](https://github.com/Martichou/rquickshare/compare/rquickshare_frontend-v1.0.0...rquickshare_frontend-v1.0.1) (2024-02-29)


### Miscellaneous Chores

* release 1.0.1 ([4a7538e](https://github.com/Martichou/rquickshare/commit/4a7538ec456684f2b6febc9f3aa9bbe5f9ffbeca))

## [1.0.0](https://github.com/Martichou/rquickshare/compare/rquickshare_frontend-v0.1.0...rquickshare_frontend-v1.0.0) (2024-02-29)


### ⚠ BREAKING CHANGES

* add frontend(tauri) & move current to core_lib

### Features

* add frontend(tauri) & move current to core_lib ([d989683](https://github.com/Martichou/rquickshare/commit/d9896837d9c00c687deb66663cd9a7ed264574ac))
* autostart at boot ([e904683](https://github.com/Martichou/rquickshare/commit/e90468311f10f95ed01ff72b98977d533dc454ee))
* export common structs to TS ([c2072b5](https://github.com/Martichou/rquickshare/commit/c2072b5260938317128c022fe8ac9ac687ea8eaf))
* front-backend communication channel ([45d9d7c](https://github.com/Martichou/rquickshare/commit/45d9d7c61761e40114a6e96b4f6bc6069fc1487f))
* improve UI & handling of requests ([4bd3e83](https://github.com/Martichou/rquickshare/commit/4bd3e832cca133dbe0f6193ffcd9b0b7be42cff2))
* move tauri to V2 & BLEA behind feature ([1187b44](https://github.com/Martichou/rquickshare/commit/1187b447336ba0446264c53fb1e86f61d3e45c35))
* prepare tauri to send cmd ([fc2a28a](https://github.com/Martichou/rquickshare/commit/fc2a28a493ada7e9ffe1611bdcb960a727ea1764))
* sending file working (and wip UI) ([53890f0](https://github.com/Martichou/rquickshare/commit/53890f08dde4261f1775f925ee6a3084fbe76eae))
* somewhat working UI channel with Rust ([6452132](https://github.com/Martichou/rquickshare/commit/6452132c976d0f02574855953a2f1b3431a5c28d))
* **ui:** tray allow to open app ([748cbbb](https://github.com/Martichou/rquickshare/commit/748cbbbccf165b4429c49fd5ee88f4e86405c11a))
* **wip:** outbound handle paired_key_result ([56e32c1](https://github.com/Martichou/rquickshare/commit/56e32c15a0fcf71eb821dca70960d9a64539a4a7))
* **wip:** try to send BLE advertisment when sending ([d8ae730](https://github.com/Martichou/rquickshare/commit/d8ae730ed09df3d28b5870e63c8a42423780d310))


### Bug Fixes

* linting ([428e141](https://github.com/Martichou/rquickshare/commit/428e141895ae687bd6e8befe30a2bfebe761dfb4))
* use vendored tls ([8c5b954](https://github.com/Martichou/rquickshare/commit/8c5b954645c996a7d2fdc2457ae257cd88cab464))
* wrong path for dep ([72ef7bc](https://github.com/Martichou/rquickshare/commit/72ef7bc26ba7c1c1ad017a232e9f4a3bd25fe8c5))

## 0.1.0 (2024-02-29)


### Features

* autostart at boot ([e904683](https://github.com/Martichou/rquickshare/commit/e90468311f10f95ed01ff72b98977d533dc454ee))
* export common structs to TS ([c2072b5](https://github.com/Martichou/rquickshare/commit/c2072b5260938317128c022fe8ac9ac687ea8eaf))
* improve UI & handling of requests ([4bd3e83](https://github.com/Martichou/rquickshare/commit/4bd3e832cca133dbe0f6193ffcd9b0b7be42cff2))
* move tauri to V2 & BLEA behind feature ([1187b44](https://github.com/Martichou/rquickshare/commit/1187b447336ba0446264c53fb1e86f61d3e45c35))
* prepare tauri to send cmd ([fc2a28a](https://github.com/Martichou/rquickshare/commit/fc2a28a493ada7e9ffe1611bdcb960a727ea1764))
* sending file working (and wip UI) ([53890f0](https://github.com/Martichou/rquickshare/commit/53890f08dde4261f1775f925ee6a3084fbe76eae))
* somewhat working UI channel with Rust ([6452132](https://github.com/Martichou/rquickshare/commit/6452132c976d0f02574855953a2f1b3431a5c28d))
* **ui:** tray allow to open app ([748cbbb](https://github.com/Martichou/rquickshare/commit/748cbbbccf165b4429c49fd5ee88f4e86405c11a))
* **wip:** outbound handle paired_key_result ([56e32c1](https://github.com/Martichou/rquickshare/commit/56e32c15a0fcf71eb821dca70960d9a64539a4a7))
* **wip:** try to send BLE advertisment when sending ([d8ae730](https://github.com/Martichou/rquickshare/commit/d8ae730ed09df3d28b5870e63c8a42423780d310))


### Bug Fixes

* linting ([428e141](https://github.com/Martichou/rquickshare/commit/428e141895ae687bd6e8befe30a2bfebe761dfb4))
* use vendored tls ([8c5b954](https://github.com/Martichou/rquickshare/commit/8c5b954645c996a7d2fdc2457ae257cd88cab464))
* wrong path for dep ([72ef7bc](https://github.com/Martichou/rquickshare/commit/72ef7bc26ba7c1c1ad017a232e9f4a3bd25fe8c5))
