A simple compressor VST using [overdrivenpotato's vst2 library](https://github.com/overdrivenpotato/rust-vst2)

This compressor has a fixed gain reduction (the `Range` parameter) instead of a traditional ratio control. When the sidechain signal is over the threshold, the signal is reduced by the given value, and the `Ratio` knob controls the sharpness of the sigmoid used to determine this.

The topology is feed-forward, and the detector is RMS.

To build

	cargo build --release

To build and bundle into a VST (OS X)

	./cargo-bundle


