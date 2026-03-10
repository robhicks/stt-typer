# stt-typer

A push-to-talk voice typing CLI for Linux. Hold **right CTRL** to speak, release to transcribe and type the result into the active window using [ydotool](https://github.com/ReimuNotMoe/ydotool). Transcription is powered by [whisper.cpp](https://github.com/ggerganov/whisper.cpp) running locally.

## Prerequisites

Fedora 43 (or similar) with a working microphone. Install the build and runtime dependencies:

```bash
# Build dependencies
sudo dnf install alsa-lib-devel clang-devel cmake gcc-c++

# Runtime dependency — virtual keyboard for typing output
sudo dnf install ydotool
sudo systemctl enable --now ydotool
```

You need access to `/dev/input/event*` devices for push-to-talk. Add yourself to the `input` group:

```bash
sudo usermod -aG input $USER
# Log out and back in for the group change to take effect
```

You also need a Rust toolchain. If you don't have one:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Download the Whisper model

stt-typer uses Whisper's `base` model by default. Download it:

```bash
mkdir -p ~/.local/share/stt-mcp
curl -fSL -o ~/.local/share/stt-mcp/ggml-base.bin \
  https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-base.bin
```

You can use a different model file with the `--model` flag or `WHISPER_MODEL_PATH` environment variable.

## Build

```bash
cargo build --release
```

The binary is written to `target/release/stt-typer`.

## Usage

```bash
target/release/stt-typer
```

Hold **right CTRL** to speak. A beep signals that recording has started. Release the key to stop recording — the audio is transcribed and typed into the active window.

### Options

```
-m, --max-duration <SECS>   Maximum seconds to record (default: 30)
-l, --language <LANG>       Language hint for Whisper (default: "en")
-M, --model <PATH>          Path to Whisper model file [env: WHISPER_MODEL_PATH]
```

### Example

```bash
# Use the large model and set Spanish as the language
target/release/stt-typer --model ~/models/ggml-large.bin --language es
```
