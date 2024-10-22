# Revolt Vortex

## Description

A new version of the voice server for Revolt.

## Stack

- [Rust](https://www.rust-lang.org/)

## Resources

### Vortex

- [Vortex Issue Board](https://github.com/revoltchat/vortex/issues)

### Revolt

- [Revolt Project Board](https://github.com/revoltchat/revolt/discussions) (Submit feature requests here)
- [Revolt Testers Server](https://app.revolt.chat/invite/Testers)
- [Contribution Guide](https://developers.revolt.chat/contributing)

## Quick Start

Get Vortex up and running locally for development.

<!-- Python gets us the desired syntax highlighting, it's shell commands. -->

```py
git clone https://github.com/revoltchat/vortex
cd vortex
cargo build
# Set the environment variables as described below
cargo run
```

## Environment Variables

| Variable              | Description                                                                                                                           | Example                          |
|-----------------------| ------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------- |
| `VORTEX_HTTP_HOST`    | The hostname to bind to.                                                                                                              | `0.0.0.0:8080` (default)         |
| `VORTEX_WS_URL`       | The websocket URL to advertise.                                                                                                       | `wss://vortex.revolt.chat`       |
| `VORTEX_MANAGE_TOKEN` | The token used for communication between Vortex and Delta.                                                                            | `<token>`                        |
| `VORTEX_RTC_MIN_PORT` | The minimum port to use for WebRTC and RTP.                                                                                           | `10000` (default)                |
| `VORTEX_RTC_MAX_PORT` | The maximum port to use for WebRTC and RTP.                                                                                           | `11000` (default)                |

## CLI Commands

| Command       | Description           |
| ------------- | --------------------- |
| `cargo build` | Build/compile Vortex. |
| `cargo run`   | Run Vortex.           |

## License

Vortex is licensed under the [GNU Affero General Public License v3.0](https://github.com/revoltchat/vortex/blob/master/LICENSE).
 
