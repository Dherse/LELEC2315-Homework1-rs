# LELEC2531 - DE-10 Nano - Hello world

This is a rust version of an assignment for the LELEC2531 course taught at UCLouvain in Belgium.
This serves as a proof of concept that rust can be utilized for this course and associated ones.

This creates an animation on the 8 FPGA LEDs on the board using the two FPGA buttons to control the speed
of the animation. The control of those buttons and the speed are done in software on the HPS. The animation
itself is implemented inside of the FPGA. This serves as an extremely basic example of how the HPS and FPGA
can communicate.

The hardware programming file is available in the `hardware` folder. Programming of the HPS can be done by debugging inside
of VSCode. Note that this expects a Linux install on the HPS and probably more configurations that were provided by Terrasic
if I am not mistaken.
