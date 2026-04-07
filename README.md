*gemi* is just another GameBoy emulator written in Rust.

Started as a learning project to get familiar with the Rust programming language,
this project evolved into a feature rich emulator with an already good accuracy.

It is still in active development aiming to support more features of the original
hardware and to improve the accuracy to be able to play the most popular games.

Check out the online demo and feel free to make suggestions about missing features
or games which are not supported properly.


[![Game Boy](doc/images/player.png)](https://christianfischer.github.io/gemi-web/)


### Feature List

| Feature                      | Status                                                                                  |
|:-----------------------------|:----------------------------------------------------------------------------------------|
| Sound                        | ✔️                                                                                      |
| Memory Bank Controller       | ✔️ MBC 1, ✔️ MBC 2, ❌ MBC 3, ✔️ MBC 5                                                   |
| Persistent Cartridge Memory  | ✔️ Supports saving and loading the cartridge RAM, if the cartridge has battery support. |
| Save/Load emulator snapshots | ❌                                                                                       |
| Serial Port / Multiplayer    | ❌ Partially to receive messages from test ROMs. Multiplayer not planned yet.            |
| GameBoy Color Support        | ✔️ Color support ❌ Double Speed mode                                                    |
| Super GameBoy support        | ❌                                                                                       |

### ToDo

* Improve accuracy, especially within the PPU
* GameBoy Color double speed mode
* Super GameBoy (2) Support
* Support to serialize the entire emulator state to allow snapshots
* Shader support to mimic the original display (LCD effect, transparent shadows)
* Controller / Rumble
* Debugger UI
* Pass more test cases
* Get the core library panic free
* Performance measurement and optimizations


### Features currently not planned <font size="1">(but never say "no")</font>
* Multiplayer via Serial Data port
* Infrared Port
* GameBoy Camera / Printer


### Project structure

* *lib/*
  * *lib/core/* - The emulator's core library with a low amount of dependencies.
    This library is aimed to be low-level. Any consumer has to implement a
    `EmulatorClient` trait to interact with the emulator.
  * *lib/gemi/* - A battery-included backend library built on the *core* library.
    This library adds the `GameBoy` struct as a default implementation of an
    `EmulatorClient` and is most suitable for common use cases.
  * *lib/mock/* - Mock implementations of `EmulatorClient` and `ImageData` traits
    to be used in unit tests.
  * *lib/utils/* - A collection of utility functions and types, which may be
    useful for frontend implementations.

* *bin/*
  * *bin/gemi-player* - The default emulator frontend, controlled via commandline
    options and using SDL to play audio and video or handle input.
  * *bin/gemi-debugger* - A simple debugger frontend to observe a game's behaviour
    during execution. This is planned to include viewing the device memory, CPU state
    and PPU/APU data like sprites and tiles.
  * *bin/wasm-player* - A wrapper around the emulator core which provides bindings
    to web assembly to allow to create a web frontend.

* *tests/*
  * *tests/shared/* - A shared library providing functionality to automatically
    download test ROMs, building a list of testcases and run each test case
    on a headless emulator instance.
  * *tests/test_suite/* - A collection of unit tests to be run via `cargo test`
    (together with other unit tests). Tests, which are know to fail will be ignored
    for normal test runs so any failing test indicates an issue in the emulator code.
  * *tests/update_test_report/* - A small binary to run all known test ROMs and generate
    a report about their results. This is used to track the emulator's accuracy
    and to find regressions.


### Library Feature Flags
The *core* and *gemi* libraries provide a set of feature flags to enable/disable
certain functionality. These flags can be used to customize the emulator's
behavior or to reduce the emulator's core size by disabling certain features.

* `std` [core] - A feature to use the standard library. This may be disabled to support
  platforms with limited capabilities like embedded devices.

* `dyn_alloc` [core] - Enables dynamic allocation support.
  This is required to hold larger data for memory to support cartridges > 32kiB
  and GameBoyColor support.

* `file_io` - Adds functions to load and write RAM and ROM images from and into files.
  This is enabled by default and may be disabled for environments which do not
  support file access.

* `cgb` - Enables GameBoyColor support.
  GBC requires additional memory, so disabling will reduce memory usage of the emulator.

* `apu` - Enables the audio processing unit.
  Disabling will remove audio support and can be used to reduce executable size.

* `serde` - Enables support for serialisation/deserialisation using the "serde" crate

* `snapshots` - Supports creating and loading snapshots of the whole emulator state.
  This will add some more dependencies like serialisation using serde,
  data compression and may increase the executable size noticeable.


### Test ROMs

The emulator is tested against several commonly used test ROMs.
Since the emulator's core functionality is placed into a dedicated library,
this allows to run the emulator headless without a UI and then check for
any success or failed conditions or compare the display output to a
reference image.

`update-test-report` will run all known test ROMs on a separate emulator instance
and check whether the test passed or failed and stores the results in the doc folder.

[List of test results](doc/test_report.md)


### Run the emulator

* **Native desktop application**

  To run the emulator as a native desktop application, you need to have
  [Rust](https://www.rust-lang.org/tools/install) installed.

  ```bash
  # Clone the repository
  git clone https://github.com/ChristianFischer/gemi.git

  # Run the emulator
  cd gemi
  cargo run --bin gemi --release -- <path-to-rom>
  ```

* **Visit the online player**

  The emulator can be played online at [https://christianfischer.github.io/gemi-web/](https://christianfischer.github.io/gemi-web/).

  This is a web assembly build of the emulator core, wrapped in a small HTML/JavaScript
  application to provide a UI and to handle audio and video output.

