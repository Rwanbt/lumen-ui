# Audio

`lumen-ui-audio` (feature `audio`) is lumen-ui's **differentiator**: DAW-grade controls no web
design system ships. Every control is painter-drawn and theme-colored — it resolves a pure recipe
from the installed theme, exactly like the core widgets, and reads no global state.

```toml
lumen-ui = { version = "1", features = ["audio"] }
```

## Controls

```rust,ignore
use lumen_ui::prelude::*;

// Knob — a 270° rotary control; drag vertically to change the value.
ui.add(Knob::new(&mut cutoff, 20.0..=20_000.0));

// Fader — a vertical level fader (max at top); click/drag to set.
ui.add(Fader::new(&mut gain_db, -60.0..=6.0));

// VuMeter — vertical level meter, green/amber/red zones + optional peak hold.
// LevelBar — horizontal level, colored by zone. Both are display-only:
// `level` (and `peak`) are 0..=1 fractions of full scale (you map dB to that).
ui.add(VuMeter::new(level).peak(peak));
ui.add(LevelBar::new(level));

// XyPad — a square 2-D control; X left→right, Y bottom→top.
ui.add(XyPad::new(&mut x, &mut y, 0.0..=1.0, 0.0..=1.0));

// Waveform — a min/max envelope of a sample buffer (samples in -1.0..=1.0),
// correct even for buffers far larger than the pixel width.
ui.add(Waveform::new(&samples));

// Transport — play/pause, stop, record; returns the action pressed this frame.
if let Some(action) = Transport::new().playing(playing).recording(recording).show(ui) {
    match action {
        TransportAction::PlayPause => playing = !playing,
        TransportAction::Stop => playing = false,
        TransportAction::Record => recording = !recording,
    }
}
```

## Theming

The meters' zones map to the **semantic tokens** (`success`/`warning`/`danger`), so a meter reads
green→amber→red under any theme. The Knob/Fader/XyPad accents follow `primary`. Swap the theme and
the whole rack restyles — see the runnable `catalogue` example:

```bash
cargo run -p lumen-ui --example catalogue --features audio,themes
```

> Validating these controls inside a real DAW (Seno / a CLAP plugin) is the remaining pre-1.0
> follow-up; a spectrum analyzer (which needs an FFT input) is intentionally left to the app.
