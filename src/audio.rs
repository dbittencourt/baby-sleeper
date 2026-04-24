use std::{fs::File, io::BufReader, path::Path};

use anyhow::{Context, Result};
use rodio::{Decoder, OutputStream, Sink, Source};

pub struct AudioLoop {
    _sink: Sink,
    _stream: OutputStream,
}

impl AudioLoop {
    pub fn start(path: &Path) -> Result<Self> {
        let file = File::open(path)
            .with_context(|| format!("failed to open sound file at {}", path.display()))?;
        let source = Decoder::new(BufReader::new(file))
            .with_context(|| format!("failed to decode sound file at {}", path.display()))?
            .repeat_infinite();

        let (_stream, stream_handle) = OutputStream::try_default()
            .context("failed to open the default audio output device")?;
        let sink = Sink::try_new(&stream_handle).context("failed to create audio sink")?;
        sink.append(source);
        sink.play();

        Ok(Self {
            _sink: sink,
            _stream,
        })
    }
}
