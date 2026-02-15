# gst-plugin-rtpmp2p

[![CI](https://github.com/Kato-emb/gst-plugin-rtpmp2p/actions/workflows/ci.yml/badge.svg)](https://github.com/Kato-emb/gst-plugin-rtpmp2p/actions/workflows/ci.yml)

GStreamer plugin for RTP MPEG-2 Program Stream (MP2P) depayloading, implemented in Rust. Extracts MPEG-2 Program Stream data from RTP packets according to [RFC 2250](https://datatracker.ietf.org/doc/html/rfc2250).

GStreamer does not include a depayloader for this format. This plugin provides one for use with legacy devices that stream MPEG-2 Program Stream over RTP.

> This plugin is not part of [gst-plugins-rs](https://gitlab.freedesktop.org/gstreamer/gst-plugins-rs) and is maintained independently.

## Elements

### rtpmp2pdepay

RTP MP2P depayloader. Extracts MPEG-2 Program Stream from RTP packets.

- **Rank**: Marginal (not preferred by autoplugging; explicit use in the pipeline is recommended)

#### Pad Templates

| Pad  | Direction | Caps |
|------|-----------|------|
| sink | Sink      | `application/x-rtp, media=video, clock-rate=90000, encoding-name=MP2P` |
| src  | Source    | `video/mpeg, mpegversion=2, systemstream=true` |

#### Pipeline Example

```sh
gst-launch-1.0 udpsrc port=5004 caps="application/x-rtp,media=video,clock-rate=90000,encoding-name=MP2P,payload=96" \
  ! rtpjitterbuffer \
  ! rtpmp2pdepay \
  ! mpegpsdemux \
  ! decodebin \
  ! autovideosink
```

> `payload=96` is an example of a dynamic payload type. Adjust this value to match the sender's SDP/configuration.

## Install

### Pre-built Binaries

Download pre-built binaries for each platform from [GitHub Releases](https://github.com/Kato-emb/gst-plugin-rtpmp2p/releases).

| Platform | File |
|----------|------|
| Linux    | `libgstrtpmp2p.so` |
| macOS    | `libgstrtpmp2p.dylib` |
| Windows  | `gstrtpmp2p.dll` |

Place the downloaded file in the GStreamer plugin directory, or set `GST_PLUGIN_PATH` (see [Setup](#setup)).

### Build from Source

#### Requirements

- Rust 1.83.0+
- GStreamer 1.20+ development libraries

#### Compile

```sh
cargo build --release
```

The shared library will be output to:

| Platform | Path |
|----------|------|
| Linux    | `target/release/libgstrtpmp2p.so` |
| macOS    | `target/release/libgstrtpmp2p.dylib` |
| Windows  | `target/release/gstrtpmp2p.dll` |

#### Test

```sh
cargo test
```

#### Setup

Copy the shared library to the GStreamer plugin directory, or set `GST_PLUGIN_PATH`:

```sh
export GST_PLUGIN_PATH=$PWD/target/release
```

Verify the plugin is loaded:

```sh
gst-inspect-1.0 rtpmp2pdepay
```

## License

MPL-2.0
