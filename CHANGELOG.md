# Changelog

## [0.11.3](https://github.com/Martichou/rquickshare/compare/v0.11.2...v0.11.3) (2024-10-08)


### Bug Fixes

* once_cell 1.20 yanked ([cd237a5](https://github.com/Martichou/rquickshare/commit/cd237a59ce3a4e795d024666687c769814f9192b))
* store in main Vue app due to upgrade to 2.0.0 of store ([7a9c0e5](https://github.com/Martichou/rquickshare/commit/7a9c0e53cef49c436c3cbd5a6ab4f1007865e8e6))
* store issues due to 2.0.0 ([91487bb](https://github.com/Martichou/rquickshare/commit/91487bbb2afa3fd3dee4137d565ac53ea501bf10))

## [0.11.2](https://github.com/Martichou/rquickshare/compare/v0.11.1...v0.11.2) (2024-08-21)


### Bug Fixes

* **ci:** upgrade (again) to googleapis/release-please-action ([52d3089](https://github.com/Martichou/rquickshare/commit/52d308989dd539fdfd92d63f8860523c9e1a500a))
* display red circle if error/cancelled/rejected, [#128](https://github.com/Martichou/rquickshare/issues/128) ([f62b9c0](https://github.com/Martichou/rquickshare/commit/f62b9c09b672b287fde0e473dc1431b35a0969b8))
* file picker different types returned ([50bc6c8](https://github.com/Martichou/rquickshare/commit/50bc6c80252fcd56632302bb82ffad6e7cd6d2de))

## [0.11.1](https://github.com/Martichou/rquickshare/compare/v0.11.0...v0.11.1) (2024-08-14)


### Bug Fixes

* opening URL permission for main ([#165](https://github.com/Martichou/rquickshare/issues/165)) ([0b50082](https://github.com/Martichou/rquickshare/commit/0b50082faba6f1f3258873f378ef93802b0e480f))
* **revert:** use old google-github-actions/release-please-action ([7584073](https://github.com/Martichou/rquickshare/commit/75840735825110ebbaf4701511b1a85b9dbc480c))

## [0.11.0](https://github.com/Martichou/rquickshare/compare/v0.10.2...v0.11.0) (2024-08-13)


### Features

* add minimize on startup option ([e084c4b](https://github.com/Martichou/rquickshare/commit/e084c4b8ddd1cb3a67c09d8d72a54eec1382566c))
* add toast notification if error while open_url ([4798d78](https://github.com/Martichou/rquickshare/commit/4798d78b7ae3d01a14c1ff61a4c68062f1566487))
* rework app closing & quit & prevent ([cdcf5f3](https://github.com/Martichou/rquickshare/commit/cdcf5f31e3e5ac74277b32d705d3526c959f1e38))
* rework texts's payload transfer info to frontend ([63464b3](https://github.com/Martichou/rquickshare/commit/63464b301e8cfe13ffe9740fab5f041bdda0eb32))
* update core_lib to report text payload type ([3fd00c4](https://github.com/Martichou/rquickshare/commit/3fd00c46b4a5e86407dd2c051bbe86b6544ed6b7))


### Bug Fixes

* also open_detached in core_lib ([c35a06c](https://github.com/Martichou/rquickshare/commit/c35a06cc34fbb31589b29fda7244faf562df2fbd))
* attempt to use open::detached for [#88](https://github.com/Martichou/rquickshare/issues/88) and [#91](https://github.com/Martichou/rquickshare/issues/91) ([22410f2](https://github.com/Martichou/rquickshare/commit/22410f24ae1195fd7ca05a76681b06215593704b))
* attempt to use tauri's shell open ([fa63e82](https://github.com/Martichou/rquickshare/commit/fa63e823a4b96a8afe99f413b952e483e71ab7bf))
* clippy error in main.rs(legacy) ([4e45096](https://github.com/Martichou/rquickshare/commit/4e450965ff54c65b64f7dd3b30c540560f72c63f))
* import not found for ToastNotification ([a5e118a](https://github.com/Martichou/rquickshare/commit/a5e118ad66b990aecec85ad800dfd51272a44f83))
* missing check for open ([b06431d](https://github.com/Martichou/rquickshare/commit/b06431d4d07cb8cfedbcddd0e5c2f34480a596e0))
* rework vue_lib plugin ([e77f7cb](https://github.com/Martichou/rquickshare/commit/e77f7cb688de41d5a84bbb10f88f67be9158d61e))
* vue_lib is not a lib anymore, I give up for now ([7c8ce74](https://github.com/Martichou/rquickshare/commit/7c8ce74d273bfe0efeb3e3590a46a6bac5e27df5))

## [0.10.2](https://github.com/Martichou/rquickshare/compare/v0.10.1...v0.10.2) (2024-07-14)


### Bug Fixes

* also build x86 macos & update deps ([037100a](https://github.com/Martichou/rquickshare/commit/037100a7fb692e405734190e31f96fd7e1cffa1d))
* legacy should not be built for macos ([9709c52](https://github.com/Martichou/rquickshare/commit/9709c52c26ab73fb07087e71a7ba0a6ab31c536a))

## [0.10.1](https://github.com/Martichou/rquickshare/compare/v0.10.0...v0.10.1) (2024-07-08)


### Bug Fixes

* avoid click propagation when clicking child ([614bcbc](https://github.com/Martichou/rquickshare/commit/614bcbcdc63c771cbb2f513050f39d8aa703b7a1))
* correctly remove device from list when sending ([758ada8](https://github.com/Martichou/rquickshare/commit/758ada81e30cc44ee92d985193a747bb1dc878b4))
* **tauri:** add default (-) signing identity for macos ([44de621](https://github.com/Martichou/rquickshare/commit/44de621f0d752c78e60a94b5bdb3298355e1eff7))

## [0.10.0](https://github.com/Martichou/rquickshare/compare/v0.9.0...v0.10.0) (2024-07-07)


### Features

* add new version notification ([4710dae](https://github.com/Martichou/rquickshare/commit/4710dae128608fd9d80abd23074920c63c1cf5ff))


### Bug Fixes

* blea use correct service_data ([121631c](https://github.com/Martichou/rquickshare/commit/121631cd5c60db8c711d2ee7eb9caa2c6bd5c2ef))
* bluetooth advertisement is working now ([0a066d8](https://github.com/Martichou/rquickshare/commit/0a066d88169650ca83e7f2138f6b28861e2bb105))
* set dbus to vendored for aarch64 ([#118](https://github.com/Martichou/rquickshare/issues/118)) ([5151f99](https://github.com/Martichou/rquickshare/commit/5151f999020ad64cc74eb9a3fa93a771627c6c66))

## [0.9.0](https://github.com/Martichou/rquickshare/compare/v0.8.2...v0.9.0) (2024-07-04)


### Features

* 'finish' macos support ([efb5982](https://github.com/Martichou/rquickshare/commit/efb598260b64d4592a0fc85f4311617d8ee22e64))
* add back name inside tray menu on v2 ([373460e](https://github.com/Martichou/rquickshare/commit/373460e0bbd7d1dc231a89e08e9df5df67390270))
* add Tauri V2 codebase ([ba4f16f](https://github.com/Martichou/rquickshare/commit/ba4f16f2284f22b3ac8e5d72e85b38c77c05d7d0))
* drag&drop working now ([3c97e6d](https://github.com/Martichou/rquickshare/commit/3c97e6dc25811dabddbae53623a285d8919a62d1))
* migrate V1 codebase for V2 ([a3226fc](https://github.com/Martichou/rquickshare/commit/a3226fc43bdfba05e09993feffcb98f61d394a8e))
* prepare for macos support ([2e7857c](https://github.com/Martichou/rquickshare/commit/2e7857c5d59045f6c216ec1eb2034a55fa2cc85c))
* support notification on macos (no action) ([8a57a11](https://github.com/Martichou/rquickshare/commit/8a57a118bc70ea7d5eade3f20e419093782ff654))
* TS migrated to Tauri V2 ([a286591](https://github.com/Martichou/rquickshare/commit/a286591d3b3250d989c8deae1be1d2ffb4f78ff1))


### Bug Fixes

* _id is not a field, use id :) ([3cce6b0](https://github.com/Martichou/rquickshare/commit/3cce6b0f5776145182fd70f2f263c0b3a176f398))
* attempt to fix pnpm issue ([bc49f65](https://github.com/Martichou/rquickshare/commit/bc49f65b150202dfee7bc87745d6b95492c56cb4))
* **ci:** debug upload name conflicted ([dacb7da](https://github.com/Martichou/rquickshare/commit/dacb7da43df4e4acf4bafa6e77de1c4fdace33dc))
* display received file.s once finished ([505258c](https://github.com/Martichou/rquickshare/commit/505258c1aae1037e573487b314db03698dbc17ed))
* fmt script go back to correct directory ([1e007db](https://github.com/Martichou/rquickshare/commit/1e007db8522c6fd761c492fa6e1a39b454e4701f))
* improve sanity wait ([c595dcd](https://github.com/Martichou/rquickshare/commit/c595dcd099909bc87fdcf29b79fa85a0acae369d))
* missing appindicator library pkg and fix glob for release ([5859434](https://github.com/Martichou/rquickshare/commit/585943473867dde3e34c5efd6c5f7c8e4aac4a8e))
* release bundle rename ([1578081](https://github.com/Martichou/rquickshare/commit/15780810a9774f5940e906e8eb0a6179563b2058))
* upload name duplicated ([20606d2](https://github.com/Martichou/rquickshare/commit/20606d2d0db215ddc6d5485c137c7998347ae941))
* useless format and not used id ([ad08e9f](https://github.com/Martichou/rquickshare/commit/ad08e9f1900be6d1642f5853a3b36f7a664daea7))

## [0.8.2](https://github.com/Martichou/rquickshare/compare/v0.8.1...v0.8.2) (2024-06-19)


### Bug Fixes

* prevent task from exiting if recv fails ([8b9d182](https://github.com/Martichou/rquickshare/commit/8b9d1824bfa7b70bef124218a56d024b9105903e))

## [0.8.1](https://github.com/Martichou/rquickshare/compare/v0.8.0...v0.8.1) (2024-06-19)


### Bug Fixes

* release artifacts ([18b37b7](https://github.com/Martichou/rquickshare/commit/18b37b7b48389ef24272813230c8b1c111d6c09e))

## [0.8.0](https://github.com/Martichou/rquickshare/compare/v0.7.1...v0.8.0) (2024-06-19)


### Features

* configure logging level with config file ([#104](https://github.com/Martichou/rquickshare/issues/104)) ([5712b3b](https://github.com/Martichou/rquickshare/commit/5712b3b7acc12030c105d93e56b51da4cbf00dbd))


### Bug Fixes

* add desktop plugs to snapcraft ([#80](https://github.com/Martichou/rquickshare/issues/80)) ([7f3dbac](https://github.com/Martichou/rquickshare/commit/7f3dbaca8d3a7a5d8625a06cdab26650c323a0c2))
* ensure correct service_data before notifying ([4027cbe](https://github.com/Martichou/rquickshare/commit/4027cbec12341ba6c4aa3ba54439266a48db069f))
* improve logging and allow to configure (env) ([1084079](https://github.com/Martichou/rquickshare/commit/10840791ec3dc4224112795771633b48202da470))
* index.ts is sorted now ([42c84d2](https://github.com/Martichou/rquickshare/commit/42c84d2675c158a3537a67d05928df1b7dc27a8b))
* make the app feel like 'real' ([3ae1396](https://github.com/Martichou/rquickshare/commit/3ae1396f05cd9e97e3775adc46b9c2b204ff2104))

## [0.7.1](https://github.com/Martichou/rquickshare/compare/v0.7.0...v0.7.1) (2024-05-01)


### Bug Fixes

* auto snap version ([8bab368](https://github.com/Martichou/rquickshare/commit/8bab368660acba9e39c5ddfba7120db15ee4024b))

## [0.7.0](https://github.com/Martichou/rquickshare/compare/v0.6.0...v0.7.0) (2024-05-01)


### Features

* add outbound progress ([#69](https://github.com/Martichou/rquickshare/issues/69)) ([4db4383](https://github.com/Martichou/rquickshare/commit/4db43835c5557077bcb60b91d3a9f2da2245b5db))

## [0.6.0](https://github.com/Martichou/rquickshare/compare/v0.5.0...v0.6.0) (2024-04-29)


### Features

* add incoming transfer progress ([#47](https://github.com/Martichou/rquickshare/issues/47)) ([f154e90](https://github.com/Martichou/rquickshare/commit/f154e9099ad720101b9d100e52bf9dd3ca87af1a))
* add option to change destination folder ([#60](https://github.com/Martichou/rquickshare/issues/60)) ([7091df0](https://github.com/Martichou/rquickshare/commit/7091df078c3dfe4424dc9217dbdfb15d10a29a6b))

## [0.5.0](https://github.com/Martichou/rquickshare/compare/v0.4.1...v0.5.0) (2024-04-02)


### Features

* allow to select a specific port number ([8188bfb](https://github.com/Martichou/rquickshare/commit/8188bfb0569707e8964ecf2490186b0da908e126))


### Bug Fixes

* go back to flex-col for itms ([3597da8](https://github.com/Martichou/rquickshare/commit/3597da86e16dd58e2f608112dc44ccd71aef061d))

## [0.4.1](https://github.com/Martichou/rquickshare/compare/v0.4.0...v0.4.1) (2024-03-10)


### Features

* support text payload (to clipboard) ([f8a7732](https://github.com/Martichou/rquickshare/commit/f8a7732fdc96e3a8cdc3e5f5becc1a1f932c68f4))


### Bug Fixes

* release-please update .lock files ([23b681c](https://github.com/Martichou/rquickshare/commit/23b681c0b0a817b7ac0eea43013e81d3fc3941f9))
* scroll when multiple requests/results ([97a1dea](https://github.com/Martichou/rquickshare/commit/97a1dea229bfb2c227a70cdcb842a7d1e8f65321))
* support sharing WiFi (act as text) ([69840b8](https://github.com/Martichou/rquickshare/commit/69840b8299ab9113e55a60f1114918ef31e8cef7))


### Miscellaneous Chores

* release 0.4.1 ([d592b8e](https://github.com/Martichou/rquickshare/commit/d592b8e56718276fbcaf398409f04bfde6c8a168))

## [0.4.0](https://github.com/Martichou/rquickshare/compare/v0.3.0...v0.4.0) (2024-03-07)


### Features

* add ability to reopen received links ([#35](https://github.com/Martichou/rquickshare/issues/35)) ([f8ee0be](https://github.com/Martichou/rquickshare/commit/f8ee0befe98294ea2a6701f5e0fb408e7e4aca1c))
* add file selector to send ([1e470cf](https://github.com/Martichou/rquickshare/commit/1e470cf70b5990c09cea346393bb1eecfaf2d997))


### Bug Fixes

* get rid of daisyui ([b94b019](https://github.com/Martichou/rquickshare/commit/b94b01970130534f0492f6cf5b65ca4a3474484b))
* realclose is inverted ([412f024](https://github.com/Martichou/rquickshare/commit/412f024c86deb80e6821f4b8402be42a4839dd54))

## [0.3.0](https://github.com/Martichou/rquickshare/compare/v0.2.0...v0.3.0) (2024-03-06)


### Features

* add ability to change device visibility ([860dff9](https://github.com/Martichou/rquickshare/commit/860dff9ec969445230e735198a4f59b4d1fb61de))
* temporarily visible mode (need ble) ([#33](https://github.com/Martichou/rquickshare/issues/33)) ([cb9b17b](https://github.com/Martichou/rquickshare/commit/cb9b17bb9f61afa716a7614e9899680bb731f79e))


### Bug Fixes

* correct color for hover on light green ([0a20588](https://github.com/Martichou/rquickshare/commit/0a20588fc673ee9e7b23da02efb5665a0374e051))

## [0.2.0](https://github.com/Martichou/rquickshare/compare/v0.1.1...v0.2.0) (2024-03-03)


### Features

* add optimization profile for release ([5203a1e](https://github.com/Martichou/rquickshare/commit/5203a1eb81965cedaf3d39850487e2bcfca41d90))
* add option to configure close action ([64c1ca9](https://github.com/Martichou/rquickshare/commit/64c1ca963a73bf0fb23561528b0ba1292c0d541b))
* implement logging into a file ([1f7c986](https://github.com/Martichou/rquickshare/commit/1f7c98668968d9c96be3c8e3ba12b4e207ff5d13))
* release process specify GLIBC version in filename ([9711ab3](https://github.com/Martichou/rquickshare/commit/9711ab30e195719df0c026657ba736b067a7db1a))
* tauri v1 & build on 20.04 ([#24](https://github.com/Martichou/rquickshare/issues/24)) ([5516783](https://github.com/Martichou/rquickshare/commit/55167836a962daef3b384ad3f32014c0511f113a))


### Bug Fixes

* parse_mdns_info fix device_type & test ([8f11566](https://github.com/Martichou/rquickshare/commit/8f115667a68bb33ad7fa916d365105022598ebc8))

## [0.1.1](https://github.com/Martichou/rquickshare/compare/v0.1.0...v0.1.1) (2024-03-02)


### Bug Fixes

* don't crash if no BT & fix some unwrap ([18d0fba](https://github.com/Martichou/rquickshare/commit/18d0fbaafc246a94b266faaa476b860f1c9ac653))
* icon in systemtray is now ok ([67be6a7](https://github.com/Martichou/rquickshare/commit/67be6a7a6ad0154215413b751435bb426a795bd1))

## 0.1.0 (2024-03-02)


### ⚠ BREAKING CHANGES

* add frontend(tauri) & move current to core_lib

### Features

* add BLE listening for nearby device sharing ([b400275](https://github.com/Martichou/rquickshare/commit/b40027548238d146b3d64c7cb3421c58c25d4a79))
* add disable/enable start on boot ([de66a09](https://github.com/Martichou/rquickshare/commit/de66a099eb38681c810c1433cb448b364ad0c9d7))
* add frontend(tauri) & move current to core_lib ([d989683](https://github.com/Martichou/rquickshare/commit/d9896837d9c00c687deb66663cd9a7ed264574ac))
* add notifications ([ba050bb](https://github.com/Martichou/rquickshare/commit/ba050bb2a9737f69e1297cd70ec5280c7deff43b))
* add TcpListener & parse proto ([76c28b7](https://github.com/Martichou/rquickshare/commit/76c28b7e4eb918e4974d29165d1a91a91941a560))
* auto_addr only V4 ([ccbc08d](https://github.com/Martichou/rquickshare/commit/ccbc08db5144008921bdc7de8088722d9e2b05f3))
* autostart at boot ([e904683](https://github.com/Martichou/rquickshare/commit/e90468311f10f95ed01ff72b98977d533dc454ee))
* display if the user rejected ([42f901f](https://github.com/Martichou/rquickshare/commit/42f901fd525b5dab1d54cefcb8467972409313c6))
* don't use hardcoded key anymore ([f36316d](https://github.com/Martichou/rquickshare/commit/f36316d230e4991dbe7275fec50697e346e70ccf))
* export common structs to TS ([c2072b5](https://github.com/Martichou/rquickshare/commit/c2072b5260938317128c022fe8ac9ac687ea8eaf))
* front-backend communication channel ([45d9d7c](https://github.com/Martichou/rquickshare/commit/45d9d7c61761e40114a6e96b4f6bc6069fc1487f))
* implement encrypted files transfer ([b1e26ce](https://github.com/Martichou/rquickshare/commit/b1e26ce688a6b203ab66f67aaabf6fe1c6b46591))
* implement key-exchange, pre-encryption ([c1b9edc](https://github.com/Martichou/rquickshare/commit/c1b9edcc8aeb28219335cab72186c4419d8d2f66))
* implement mDNS service broadcast ([ebd2386](https://github.com/Martichou/rquickshare/commit/ebd23866930cd99f5a0e865f24c34760a4a228b5))
* improve UI & handling of requests ([4bd3e83](https://github.com/Martichou/rquickshare/commit/4bd3e832cca133dbe0f6193ffcd9b0b7be42cff2))
* **improve:** stabilise UI & communication ([7120162](https://github.com/Martichou/rquickshare/commit/712016280a33542774458cf25ae8666db803e1dd))
* init ([143df16](https://github.com/Martichou/rquickshare/commit/143df16153c6ddcd03876aa24f1f57cff714c288))
* make the core_lib actually being a lib ([becf516](https://github.com/Martichou/rquickshare/commit/becf5164e8f5a47e8bb22617ab947cf85c0e1fac))
* move tauri to V2 & BLEA behind feature ([1187b44](https://github.com/Martichou/rquickshare/commit/1187b447336ba0446264c53fb1e86f61d3e45c35))
* prepare tauri to send cmd ([fc2a28a](https://github.com/Martichou/rquickshare/commit/fc2a28a493ada7e9ffe1611bdcb960a727ea1764))
* real hostname in UI ([f9661fd](https://github.com/Martichou/rquickshare/commit/f9661fd52443413e041a6b5a5c3183cc10a7bcbb))
* release process ([b989b05](https://github.com/Martichou/rquickshare/commit/b989b05e6151d618d77b8e6379518eb55f9c1b1a))
* remove random mdns name ([6d83183](https://github.com/Martichou/rquickshare/commit/6d83183c0bb3df3f70276d2d6ad72b3820541a8c))
* resend broadcast mdns if nearby sharing ([2aa4701](https://github.com/Martichou/rquickshare/commit/2aa4701dd3d3ae447f1035ede0dbf0b56e777b63))
* sending file working (and wip UI) ([53890f0](https://github.com/Martichou/rquickshare/commit/53890f08dde4261f1775f925ee6a3084fbe76eae))
* somewhat working UI channel with Rust ([6452132](https://github.com/Martichou/rquickshare/commit/6452132c976d0f02574855953a2f1b3431a5c28d))
* **ui:** add version info ([db14a39](https://github.com/Martichou/rquickshare/commit/db14a396c127dc793d98a966e5f34b96460c15b9))
* **ui:** tray allow to open app ([748cbbb](https://github.com/Martichou/rquickshare/commit/748cbbbccf165b4429c49fd5ee88f4e86405c11a))
* update transfer_metadata for future use ([cde67c0](https://github.com/Martichou/rquickshare/commit/cde67c04a55c4f06c5c3aaa65ff87810b56f39af))
* **wip:** implement base for outbound transfer ([d4f2da5](https://github.com/Martichou/rquickshare/commit/d4f2da5842033d893d62b9b1a54fbc979a640b16))
* **wip:** outbound handle connection response ([21c7dd8](https://github.com/Martichou/rquickshare/commit/21c7dd8e7e6b5075fc155f824918a1892d3a92a4))
* **wip:** outbound handle paired_key_result ([56e32c1](https://github.com/Martichou/rquickshare/commit/56e32c15a0fcf71eb821dca70960d9a64539a4a7))
* **wip:** try to send BLE advertisment when sending ([d8ae730](https://github.com/Martichou/rquickshare/commit/d8ae730ed09df3d28b5870e63c8a42423780d310))


### Bug Fixes

* apply autostart if no value ([87f49a8](https://github.com/Martichou/rquickshare/commit/87f49a854ae486cbdf9918c387ec95dfc3fade8c))
* linting ([428e141](https://github.com/Martichou/rquickshare/commit/428e141895ae687bd6e8befe30a2bfebe761dfb4))
* notifications blocked the thread ([566894c](https://github.com/Martichou/rquickshare/commit/566894c2fd754e4e586d091ef4a8b602528753e5))
* only handle own id for inbound channel ([ed7453b](https://github.com/Martichou/rquickshare/commit/ed7453baae966284afcc92d55b3f39c36a1bcacc))
* release all linux bundles ([ade8a94](https://github.com/Martichou/rquickshare/commit/ade8a94164b27087abaebddff59c162b5d87f0c6))
* release: correct process (attempt[#1](https://github.com/Martichou/rquickshare/issues/1)) ([32e1939](https://github.com/Martichou/rquickshare/commit/32e19392763084111ec67ee25f8ab4dec85e8181))
* **ui:** improve style ([d402374](https://github.com/Martichou/rquickshare/commit/d40237413239ba07f3aee4b66deaedc42640bc7c))
* unlisten & blur/focus fixes ([a6f52b4](https://github.com/Martichou/rquickshare/commit/a6f52b41468b3f448d4f234fa34b3c57b92f59ef))
* use vendored tls ([8c5b954](https://github.com/Martichou/rquickshare/commit/8c5b954645c996a7d2fdc2457ae257cd88cab464))
* wrong path for dep ([72ef7bc](https://github.com/Martichou/rquickshare/commit/72ef7bc26ba7c1c1ad017a232e9f4a3bd25fe8c5))
