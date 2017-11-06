#[macro_use] extern crate vst2;

use vst2::plugin::{Category, Info, Plugin};
use vst2::buffer::AudioBuffer;

type SamplePair = (f32, f32);

struct DuckComp {
//    buffers: Vec<VecDeque<SamplePair>>,
    attack: f32,
    release: f32,
    threshold: f32,
    makeup: f32,
    Q: f32
}

impl Default for DuckComp {
    fn default() -> DuckComp{
        DuckComp::new(0.3, 0.4, 0.1, 0.25)
    }
}

impl DuckComp {
    fn new(attack: f32, release: f32, threshold: f32, makeup: f32) -> DuckComp{
        // let mut buffer = VecDeque::
        DuckComp{
            attack: attack,
            release: release,
            threshold: threshold,
            makeup: makeup,
            Q: 0.0f32
        }
    }

    fn d_q(&self, x: f32) -> f32 {
        (self.attack*x - (self.attack + self.release)*self.Q)/100.
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
            parameters: 4,
            category: Category::Effect,

            ..Default::default()
        }
    }

    fn get_parameter(&self, index: i32) -> f32 {
        match index {
            0 => self.attack,
            1 => self.release,
            2 => self.threshold,
            4 => self.makeup,
            _ => 0.0f32
        }
    }

    fn get_parameter_text(&self, index: i32) -> String {
        match index{
            0 => format!("{}", self.attack),
            1 => format!("{}", self.release),
            2 => format!("{}", self.threshold),
            3 => format!("{}", self.makeup),
            _ => "".to_string()
        }
    }

    fn get_parameter_name(&self, index: i32) -> String {
        match index{
            0 => "Attack".to_string(),
            1 => "Release".to_string(),
            2 => "Threshold".to_string(),
            3 => "Make-up gain".to_string(),
            _ => "".to_string()
        }
    }

    fn set_parameter(&mut self, index: i32, val: f32){
        match index{
            0 => self.attack=val,
            1 => self.release=val,
            2 => self.threshold=val,
            3 => self.makeup=val,
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
            let gain = (1.0f32 - self.Q).max(0f32);
            *right_out = *right_in * gain;
            *left_out = *left_in * gain;
            let sidechain_signal: f32 = rectify(50. * self.threshold*0.5f32*(*left_out + *right_out));
            self.Q = self.Q + self.d_q(sidechain_signal);
            *left_out = *left_out * 4. *self.makeup;
            *right_out = *right_out * 4. *self.makeup;
        }
    }
}

fn rectify(x: f32) -> f32{
    x.abs().max(0.1f32)
    // (x.exp() + 1.0f32).ln()
}

plugin_main!(DuckComp);

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
