# rogauracore-gui

A small GTK4 desktop frontend for the `rogauracore` CLI on ASUS ROG laptops.

## What it does

This app exposes the locally installed `rogauracore` commands in a GUI so you do not need to remember the CLI syntax.

Current supported commands:

- `single_static COLOR`
- `single_breathing COLOR1 [COLOR2] [SPEED]`
- `single_pulsing COLOR SPEED`
- `single_colorcycle SPEED`
- `multi_static COLOR1 COLOR2 COLOR3 COLOR4`
- `multi_breathing COLOR1 COLOR2 COLOR3 COLOR4 SPEED`
- `rainbow [SPEED]`
- `brightness BRIGHTNESS`

## Project structure

```text
src/
  app/          GTK application setup and window composition
  rogauracore/  command models and process execution
  ui/           per-command pages and reusable widgets
  lib.rs        module wiring
  main.rs       app entrypoint
```

## Run

```bash
cargo run
```

The app expects the `rogauracore` binary to already be available on your system path.
