# Figma Workflow

Sync design tokens from Figma to OxideUI using the Oxide CLI.

## Prerequisites

- **Figma account** with access to the design file
- **Personal access token** — [Figma Settings → Personal access tokens](https://www.figma.com/settings)
- **File key** — From the Figma file URL: `https://www.figma.com/file/FILE_KEY/...`

## Getting a Figma Token

1. Go to [Figma Settings](https://www.figma.com/settings)
2. Scroll to **Personal access tokens**
3. Click **Generate new token**
4. Copy the token (shown only once)

## CLI Setup

```bash
# Install the CLI
cargo install --path crates/oxide-cli

# Save your Figma token
oxide config set <YOUR_TOKEN>

# Verify
oxide config show
```

## Figma Variable Organization

Organize variables in Figma for best results:

- **Collections**: `primitives`, `semantic`, or theme names
- **Modes**: Optional (e.g. `light`, `dark`) — each mode can map to a theme
- **Naming**: `category.token` (e.g. `color.interactive.default`)

### Recommended Structure

| Collection | Variables |
|------------|-----------|
| colors | surface_primary, text_primary, interactive_default, ... |
| spacing | none, xs, sm, md, lg, xl, xxl |
| radius | none, sm, md, lg, xl, full |
| typography | font_family, font_size_*, line_height_* |

## Syncing

```bash
# Sync from Figma to figma/tokens.json
oxide sync --file-key <FILE_KEY>

# Or set FIGMA_FILE_KEY env var
export FIGMA_FILE_KEY=abc123
oxide sync
```

## JSON Format

The `figma/tokens.json` structure:

```json
{
  "version": "1.0",
  "themes": {
    "openai": {
      "name": "OpenAI",
      "colors": {
        "surface_primary": "#FFFFFF",
        "interactive_default": "#10A37F",
        ...
      },
      "spacing": { "none": 0, "xs": 4, "sm": 8, ... },
      "radius": { "none": 0, "sm": 4, "md": 8, ... },
      "typography": { "font_family": "Inter", ... }
    }
  }
}
```

## Generating Code

```bash
# Generate Rust theme code from tokens.json
oxide generate

# Custom paths
oxide generate --input figma/tokens.json --output src/themes.rs
```

## Watch Mode

Auto-sync when Figma changes:

```bash
oxide watch --file-key <FILE_KEY> --interval 30
```

## Validation

```bash
oxide validate
```

Checks:
- `themes` object exists
- Each theme has `name`, `colors`, `spacing`, `radius`, `typography`
- Required color fields present

## Troubleshooting

| Issue | Solution |
|-------|----------|
| **401 Unauthorized** | Check token; ensure `file_variables:read` scope |
| **404 Not Found** | Verify file key; ensure file is accessible |
| **Empty themes** | Figma Variables API may return different structure; use `figma/tokens.json` as fallback |
| **Validation fails** | Run `oxide validate` for specific errors; ensure all required fields in each theme |
