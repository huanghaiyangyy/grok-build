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

## Linux CI artifact (Ubuntu 22.04)

Workflow template: [`ci/build-linux.yml`](ci/build-linux.yml)

> GitHub OAuth from this assistant cannot write `.github/workflows/` without the
> `workflow` scope. Enable CI once on the web:

1. Open https://github.com/huanghaiyangyy/grok-build/new/feat/dracula-and-terminal-transparency?filename=.github/workflows/build-linux.yml
2. Paste the contents of `ci/build-linux.yml`
3. Commit on the feature branch
4. **Actions → Build Linux (Ubuntu 22.04) → Run workflow**

Artifact name: `grok-linux-x86_64-ubuntu22.04` (contains `grok`, `xai-grok-pager`, checksums).

Replace official binary on Ubuntu 22.04:

```sh
which grok
sudo cp "$(which grok)" "$(which grok).official.bak"
sudo cp ./grok "$(which grok)"
grok --version
```

Or later, re-authorize `gh` with workflow scope so pushes to `.github/workflows/` work:
`gh auth refresh -h github.com -s workflow`
