# Fork notes (huanghaiyangyy/grok-build)

## Dracula theme

Two new themes are available in `/theme` and `[ui] theme = …`:

| Name | Config value | Notes |
|------|----------------|-------|
| Dracula | `dracula` | Classic opaque Dracula palette |
| Dracula (Terminal Transparent) | `dracula-transparent` (aliases: `dracula-ghostty`, `dracula-term`) | Same accents; backgrounds are `Reset` so Ghostty / transparent terminals show through |

Example `~/.grok/config.toml`:

```toml
[ui]
theme = "dracula-transparent"
```

Or opaque Dracula in fullscreen:

```toml
[ui]
theme = "dracula"
```

## Following terminal transparency (Ghostty, etc.)

Upstream already supports this via **minimal screen mode**, which paints no opaque app background and uses the terminal-native palette:

```toml
[ui]
screen_mode = "minimal"
```

In the TUI: `/screen-mode minimal` (or the settings UI).

Recommended Ghostty combo:

```toml
[ui]
theme = "dracula-transparent"
# optional; strongest transparency inheritance:
# screen_mode = "minimal"
```

Make sure Ghostty itself has a transparent background (window opacity / background opacity in Ghostty config).
