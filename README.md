# dlb_mp4base

The Dolby MP4 streaming muxer (dlb_mp4base) is a software implementation of a muxer of fragmented or unfragmented ISO base media file format (mp4). It supports muxing of Dolby Digital (AC-3), Dolby Digital Plus (E-AC-3), and Dolby AC-4 audio formats as well as Dolby Vision.

## Getting Started

These instructions will help you get a copy of the project up and running on your local machine for development and testing purposes.

### Folder Structure

The "dlb_mp4base" folder consists of:

- README.md         This file.
- doc/              Doxygen documentation of the dlb_mp4base.
- frontend/         MP4Muxer frontend with corresponding EMA interface as source code.
- include/          Necessary header files of the dlb_mp4base library.
- make/             Makefiles and Visual Studio projects/solutions for building the Dolby MP4 multiplexer library with frontends and test harnesses.
- src/              Contains the MP4 multiplexer source code.
- test/             Test harnesses for unit and developer system tests including test signals.

### Prerequisites

For Windows platform development, Visual Studio 2010 must be installed with SP1.

### Building instructions

#### Using the Rust, cmake and Visual Studio tools (on Windows)

    After cloning the dlb_mp4base repository to your local machine, go to the:
    "cd mp4muxer2\make\mp4muxer.library"

    Then build one of the following make targets:
    "cmake -S . -B ..\\..\\build"
    "cd ..\\..\\build"
    "msbuild mp4muxer.sln -target:mp4muxer:Rebuild /property:Configuration=Release"
    "copy .\\Release\\mp4muxer.lib .\\"
    "cd ..\\mp4muxer2.rs"
    "cargo update"
    "cargo build --release"

#### Using the Rust, cmake (on Linux and MacOS)

    After cloning the dlb_mp4base repository to your local machine, go to the:
    "cd mp4muxer2/make/mp4muxer.library"

    Then build one of the following make targets:
    "cmake -DCMAKE_BUILD_TYPE=Release -S . -B ../../build"
    "cd ../../build"
    "make"
    "cd ../mp4muxer2.rs"
    "cargo update"
    "cargo build --release"

## Release Notes

See the [Release Notes](ReleaseNotes.md) file for details

## License

This project is licensed under the BSD-3 License - see the [LICENSE](LICENSE) file for details


