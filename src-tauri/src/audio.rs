use crate::dsp::DspProcessor;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use ringbuf::traits::{Consumer, Observer, Producer, Split};
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

        // Create a ring buffer
        let buffer_size = sample_rate * channels * 2;
        let rb = HeapRb::<f32>::new(buffer_size);
        let (mut prod, mut cons) = rb.split();

        // Spawn DSP thread
        thread::spawn(move || {
            println!("DSP thread started, waiting for audio...");
            let fft_size = 512;
            let mut dsp = DspProcessor::new(fft_size, sample_rate as f32);
            let mut frame_buf = vec![0.0; fft_size];

            let mut last_print = std::time::Instant::now();
            let mut max_rms: f32 = 0.0;

            loop {
                if cons.occupied_len() >= fft_size {
                    // Read a frame
                    let mut read = 0;
                    while read < fft_size {
                        if let Some(sample) = cons.try_pop() {
                            frame_buf[read] = sample;
                            read += 1;
                        }
                    }

                    let rms = dsp.process_frame_and_get_rms(&frame_buf);
                    if rms > max_rms {
                        max_rms = rms;
                    }
                } else {
                    thread::sleep(Duration::from_millis(5));
                }

                if last_print.elapsed() >= Duration::from_secs(1) {
                    println!(
                        "[HEARTBEAT] Audio thread alive. Max RMS last sec: {:.6}",
                        max_rms
                    );
                    max_rms = 0.0;
                    last_print = std::time::Instant::now();
                }
            }
        });

        let err_fn = move |err| {
            eprintln!("an error occurred on stream: {}", err);
        };

        let stream = match config.sample_format() {
            cpal::SampleFormat::F32 => device.build_input_stream(
                config.into(),
                move |data: &[f32], _: &cpal::InputCallbackInfo| {
                    for chunk in data.chunks(channels) {
                        if !prod.is_full() {
                            // Take only the first channel to make it mono
                            let _ = prod.try_push(chunk[0]);
                        }
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
