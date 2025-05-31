# rust_redux_example

This repository contains an example application to get familiar with the redux architecture
in rust.

As example application, a small audio mixer is implemented, which can get the input signal of
different input devices and can mix them together into an output device.
Each input device can get a separate gain which increases or decreases the output signal
of this device.
And each input device can be muted.