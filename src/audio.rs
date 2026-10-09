use crate::CliArgs;

const SAMPLE_RATE: u32 = 44_100;

pub fn spawn_audio_thread(args: &CliArgs) {
    let wav = if let Some(filename) = &args.wav {
        let spec = hound::WavSpec {
            channels: 2,
            sample_rate: SAMPLE_RATE,
            bits_per_sample: 16,
            sample_format: hound::SampleFormat::Int,
        };
        let writer = hound::WavWriter::create(filename, spec).unwrap();
        Some(writer)
    } else {
        None
    };

    let Ok(stream) = rodio::OutputStreamBuilder::open_default_stream() else {
        eprintln!("WARNING: audio device is not available, sound will be disabled");
        return;
    };
    let mixer = stream.mixer();
    let source = AudioReader { wav };
    mixer.add(source);
    // Don't drop audio stream, keep playing audio as long as the process is running.
    std::mem::forget(stream);
}

struct AudioReader {
    wav: Option<hound::WavWriter<std::io::BufWriter<std::fs::File>>>,
}

impl rodio::Source for AudioReader {
    fn current_span_len(&self) -> Option<usize> {
        None
    }

    fn channels(&self) -> u16 {
        2
    }

    fn sample_rate(&self) -> u32 {
        SAMPLE_RATE
    }

    fn total_duration(&self) -> Option<core::time::Duration> {
        None
    }
}

impl Iterator for AudioReader {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        let mut buf: [i16; 1] = [0];
        firefly_runtime::audio::exec_external(|manager| {
            manager.write(&mut buf);
        });
        let s = buf[0];
        if let Some(wav) = self.wav.as_mut() {
            wav.write_sample(s).unwrap();
        }
        let s = f32::from(s) / f32::from(i16::MAX);
        Some(s)
    }
}
