# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

A push-to-talk voice typing CLI for Linux. Hold right CTRL to speak, release to transcribe via whisper.cpp, and type the result into the active window using ydotool.

## Build & Run

```bash
# Build (requires: alsa-lib-devel clang-devel cmake gcc-c++)
cargo build --release

# The Whisper model must exist at ~/.local/share/stt-mcp/ggml-base.bin
# or set WHISPER_MODEL_PATH to a custom location

# Run
target/release/stt-typer

# Container build
podman build -f Containerfile -t stt-typer .
```

There are no tests in this project currently.

## Architecture

Four source files, each with a single responsibility:

- **`src/main.rs`** — CLI entry point using `clap`. Parses args, loads the Whisper model once, then loops: wait for right CTRL press, record audio until release, transcribe, type result via `ydotool`. Also handles ydotool socket detection and plays a beep on recording start.

- **`src/audio.rs`** — Audio capture via `cpal`. `record()` opens the default input device and records for a fixed duration. `record_until_stopped()` records until an `AtomicBool` is set. Both return mono 16kHz f32 samples (what Whisper expects). Supports F32 and I16 sample formats.

- **`src/keyboard.rs`** — Keyboard input via `evdev`. `find_keyboard_devices()` scans for devices supporting KEY_RIGHTCTRL. `wait_for_right_ctrl()` and `wait_for_right_ctrl_release()` poll for key press/release in non-blocking mode.

- **`src/transcribe.rs`** — Whisper inference via `whisper-rs`. Exposes `create_context` (loads model once) and `transcribe_with_context` (runs inference on a context).

## Key Dependencies

- `whisper-rs` — Rust bindings to whisper.cpp (requires cmake/clang at build time)
- `cpal` — Cross-platform audio input (requires alsa-lib-devel on Linux)
- `evdev` — Linux input event device reading (requires user in `input` group for `/dev/input` access)
- `clap` — CLI argument parsing
