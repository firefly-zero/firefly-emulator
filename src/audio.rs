#![expect(static_mut_refs)]
use crate::CliArgs;
use std::collections::VecDeque;
use std::sync::Mutex;
use std::time::Duration;

const SAMPLE_RATE: u32 = 44_100;
type Buf = [i16; 4096];
static mut BUFS: Mutex<VecDeque<Buf>> = Mutex::new(VecDeque::new());

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
    let source = AudioReader {
        wav,
        buf: VecDeque::new(),
    };
    mixer.add(source);
    let thread = std::thread::spawn(fill_buf);
    // Don't drop audio stream, keep playing audio as long as the process is running.
    std::mem::forget(stream);
    std::mem::forget(thread);
}

fn fill_buf() {
    loop {
        push_buf();
    }
}

fn push_buf() {
    let bufs = unsafe { BUFS.lock() }.unwrap();
    let full = bufs.len() > 2;
    drop(bufs);
    if full {
        std::thread::sleep(Duration::from_millis(1));
        return;
    }

    let mut buf: Buf = [0; _];
    firefly_runtime::audio::exec_external(|manager| {
        manager.write(&mut buf[..]);
    });
    let mut bufs = unsafe { BUFS.lock() }.unwrap();
    bufs.push_back(buf);
}

struct AudioReader {
    wav: Option<hound::WavWriter<std::io::BufWriter<std::fs::File>>>,
    buf: VecDeque<i16>,
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
        if self.buf.is_empty() {
            let mut bufs = unsafe { BUFS.lock() }.unwrap();
            if let Some(buf) = bufs.pop_front() {
                self.buf.extend(buf);
            } else {
                return Some(0.0);
            }
        }

        let s = self.buf.pop_front().unwrap();
        if let Some(wav) = self.wav.as_mut() {
            wav.write_sample(s).unwrap();
        }
        let s = f32::from(s) / f32::from(i16::MAX);
        Some(s)
    }
}
