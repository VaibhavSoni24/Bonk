use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use ringbuf::traits::{Observer, Producer, Split};
use ringbuf::HeapRb;
use std::thread;
use std::time::Duration;

pub fn start_audio_capture() {
    thread::spawn(|| {
        let host = cpal::default_host();
        let device = host
            .default_input_device()
            .expect("No default input device available");
        println!("Input device selected");

        let config = device
            .default_input_config()
            .expect("Failed to get default input config");
        let sample_rate = config.sample_rate() as usize;
        let channels = config.channels() as usize;

        println!(
            "Default config: {} Hz, {} channels, {:?}",
            sample_rate,
            channels,
            config.sample_format()
        );

        // Create a ring buffer for ~2 seconds of audio
        let buffer_size = sample_rate * channels * 2;
        let rb = HeapRb::<f32>::new(buffer_size);
        let (mut prod, mut _cons) = rb.split();

        let err_fn = move |err| {
            eprintln!("an error occurred on stream: {}", err);
        };

        let stream = match config.sample_format() {
            cpal::SampleFormat::F32 => device.build_input_stream(
                config.config(),
                move |data: &[f32], _: &cpal::InputCallbackInfo| {
                    // Push to ring buffer (discarding oldest if full just for now to prevent blocking)
                    // In Phase 3, the DSP thread will consume this.
                    let mut max_amp = 0.0_f32;
                    for &sample in data {
                        // Normally we would just push, and if full, the consumer is too slow.
                        // For phase 2 validation, we just ensure it doesn't crash.
                        if !prod.is_full() {
                            let _ = prod.try_push(sample);
                        }

                        let abs = sample.abs();
                        if abs > max_amp {
                            max_amp = abs;
                        }
                    }

                    // Debug-only live level output if amplitude is noticeable
                    if max_amp > 0.05 {
                        println!("Audio detected, max amplitude: {:.3}", max_amp);
                    }
                },
                err_fn,
                None,
            ),
            _ => panic!("Unsupported sample format. Only F32 is supported for now."),
        }
        .expect("Failed to build input stream");

        stream.play().expect("Failed to start audio stream");

        // Keep the audio thread alive
        loop {
            thread::sleep(Duration::from_secs(1));
        }
    });
}
