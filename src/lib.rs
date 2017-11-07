#[macro_use] extern crate vst2;

use vst2::plugin::{Category, Info, Plugin};
use vst2::buffer::AudioBuffer;

type SamplePair = (f32, f32);

const SAMPLERATE: f32 = 41400.;
const RMS_FILTER_FACTOR: f32 = 1./(0.01*SAMPLERATE); // 1/(10 ms)

struct DuckComp {
    attack: f32,
    release: f32,
    threshold: f32,
    ratio: f32,
    makeup: f32,
    range: f32,
    Q: f32,
    rms_filter_q: f32
}

impl Default for DuckComp {
    fn default() -> DuckComp{
        DuckComp::new(0.03, 0.4, 1.0, 0.1, 1., 0.)
    }
}

impl DuckComp {
    fn new(attack: f32, release: f32, threshold: f32, ratio: f32, makeup: f32, range: f32) -> DuckComp{
        DuckComp{
            attack: attack,
            release: release,
            threshold: threshold,
            ratio: ratio,
            makeup: makeup,
            range: range,
            Q: 0.0f32,
            rms_filter_q: 0f32
        }
    }

    fn d_q(&self, x: f32) -> f32 {
        // Time parameters in seconds
        let current_in = if x > self.Q {
            (x - self.Q)/self.attack
        }else{
            0.
        };
        (current_in - self.Q/self.release)/SAMPLERATE
    }

    fn gain(&self) -> f32{
        // 1./(1.+self.ratio*self.Q)
        let arg = self.ratio*(self.Q - 0.5);
        1. - (1. - self.range)*arg.exp()/(1. + arg.exp())
    }

    fn sidechain(&mut self, x: f32) -> f32 {
        let squared = x*x;
        self.rms_filter_q = squared*RMS_FILTER_FACTOR + self.rms_filter_q*(1.-RMS_FILTER_FACTOR);
        (self.rms_filter_q.sqrt()/self.threshold - 1.).max(0.)
    }
}

impl Plugin for DuckComp {
    fn get_info(&self) -> Info {
        Info {
            name: "Duck Comp".to_string(),
            vendor: "Trls Audio".to_string(),
            unique_id: 61692745,
            version: 0001,
            inputs: 2,
            outputs: 2,
            parameters: 6,
            category: Category::Effect,

            ..Default::default()
        }
    }

    fn get_parameter(&self, index: i32) -> f32 {
        match index {
            0 => self.attack,
            1 => self.release,
            2 => self.threshold,
            3 => self.ratio,
            4 => self.makeup,
            5 => self.range,
            _ => 0.0f32
        }
    }

    fn get_parameter_text(&self, index: i32) -> String {
        match index{
            0 => format!("{:.2} ms", self.attack*1000.),
            1 => format!("{:.2} ms", self.release*1000.),
            2 => format!("-{:.2} dB", 20.*self.threshold.log10()),
            3 => format!("1:{:.2}", self.ratio),
            4 => format!("{:.2} dB", gain_to_db(self.makeup)),
            5 => format!("{:.2} dB", gain_to_db(self.range)),
            _ => "".to_string()
        }
    }

    fn get_parameter_name(&self, index: i32) -> String {
        match index{
            0 => "Attaczk".to_string(),
            1 => "Release".to_string(),
            2 => "Threshold".to_string(),
            3 => "Ratio".to_string(),
            4 => "Make-up gain".to_string(),
            5 => "Range".to_string(),
            _ => "".to_string()
        }
    }

    fn set_parameter(&mut self, index: i32, val: f32){
        match index{
            0 => self.attack=0.005 + val*(0.300 - 0.001),
            1 => self.release=0.010 + val*(1. - 0.010),
            2 => self.threshold=(10f32).powf(2.*(val-1.)),
            3 => self.ratio=val*99. + 1.,
            4 => self.makeup=db_to_gain(-12. + val*(20. - (-12.))),
            5 => self.range=db_to_gain((val-1.)*60.),
            _ => ()
        }
    }

    fn process(&mut self, buffer: AudioBuffer<f32>){
        let (inputs, mut outputs) = buffer.split();
        // Iterate over inputs as (&f32, &f32)
        let stereo_in = match inputs.split_at(1) {
            (l, r) => l[0].iter().zip(r[0].iter())
        };

        // Iterate over outputs as (&mut f32, &mut f32)
        let stereo_out = match outputs.split_at_mut(1) {
            (l, r) => l[0].iter_mut().zip(r[0].iter_mut())
        };

        for ((left_in, right_in), (left_out, right_out)) in stereo_in.zip(stereo_out) {
            let sidechain_signal = self.sidechain(0.5f32*(*left_in + *right_in));
            self.Q = self.Q + self.d_q(sidechain_signal);
            let gain = self.gain();
            *right_out = *right_in * gain * self.makeup;
            *left_out = *left_in * gain * self.makeup;
        }
    }
}

fn rectify(x: f32) -> f32{
    x.abs().max(0.1f32)
    // (x.exp() + 1.0f32).ln()
}

fn db_to_gain(x: f32) -> f32{
    (10f32).powf(x/20.)
}

fn gain_to_db(x: f32) -> f32 {
    20.*x.log10()
}

plugin_main!(DuckComp);

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
