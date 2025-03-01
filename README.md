# pdfannotator

A simple pdf annotator that stores annotation in a separate file.

## Dependenciew

See https://v2.tauri.app/start/prerequisites/#system-dependencies.

## Build

```
pnpm i
pnpm tauri build
```

## Known issues

On Linux WebKit might not work properly, you need to set some environment variable:

```bash
WEBKIT_DISABLE_DMABUF_RENDERER=1 pdfannotator xxx.pdf
# OR
WEBKIT_DISABLE_COMPOSITING_MODE=1 pdfannotator xxx.pdf
```