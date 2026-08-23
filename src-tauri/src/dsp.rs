use rustfft::{num_complex::Complex, FftPlanner};

pub struct DspProcessor {
    planner: FftPlanner<f32>,
    fft_size: usize,
    sample_rate: f32,
}

impl DspProcessor {
    pub fn new(fft_size: usize, sample_rate: f32) -> Self {
        Self {
            planner: FftPlanner::new(),
            fft_size,
            sample_rate,
        }
    }

    pub fn process_frame(&mut self, samples: &[f32]) {
        let rms = Self::calculate_rms(samples);

        // Trigger threshold: log events above ambient noise
        if rms > 0.02 {
            let centroid = self.spectral_centroid(samples);
            println!("DSP_EVENT | RMS: {:.5} | Centroid: {:.1} Hz", rms, centroid);
        }
    }

    fn calculate_rms(samples: &[f32]) -> f32 {
        if samples.is_empty() {
            return 0.0;
        }
        let sum_sq: f32 = samples.iter().map(|&x| x * x).sum();
        (sum_sq / samples.len() as f32).sqrt()
    }

    fn spectral_centroid(&mut self, samples: &[f32]) -> f32 {
        let mut buffer: Vec<Complex<f32>> = samples
            .iter()
            .take(self.fft_size)
            .map(|&x| Complex { re: x, im: 0.0 })
            .collect();

        buffer.resize(self.fft_size, Complex { re: 0.0, im: 0.0 });

        let fft = self.planner.plan_fft_forward(self.fft_size);
        fft.process(&mut buffer);

        let mut num = 0.0;
        let mut den = 0.0;

        let half_size = self.fft_size / 2;
        let freq_resolution = self.sample_rate / self.fft_size as f32;

        for i in 1..half_size {
            let magnitude = buffer[i].norm();
            let freq = i as f32 * freq_resolution;
            num += freq * magnitude;
            den += magnitude;
        }

        if den < 1e-6 {
            return 0.0;
        }

        num / den
    }
}
