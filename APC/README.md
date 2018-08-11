# Auxilaries Power Controller (APU)

This is an ARM Cortex M3 controlling the power output to the auxilaries of a motorcycle. The
MCU reads input from the driver controls, processes these and switches the according auxilaries
on or off (Headlight, Brakelight, Turn signals, etc). The output is controlled via BTS432E2 
powerfets with on-die protection for all the things (ESD, shorts, etc) as well as a diagnosis pin.

The logic is implemented in rust

* Read inputs from the driver controls
* Determine the on/off state of the auxilaries
* Switch the power output to the auxilaries
* Read diagnosis from all output channels
* handle possible errors
* (send telemtric information to ???)

## Inputs

The input is handled by a BD3376EFV-CE2 (Multiple Inpout Switch Monitor). All external driver controls 
are connected via this chip. It is responsible for securing against hazards (ESD, shorts,etc) and dispatches
the state of the inputs to the ARM Cortex M3 via SPI.

* Killswitch
* Turn Signal Left
* Turn Signal Right
* Hazard
* Headlight
* Fullbeam
* Brake signal Front
* Brake signal Rear
* Sidestand
* Horn

## Outputs

The ouput has 12 channel. The software takes completely care of the channel
usage, so they are fully programmable.

* Turn signal left front
* Turn signal right front
* Turn signal left rear
* Turn signal right rear
* Headlight parking
* Headlight lower beam
* Headlight full beam
* Rearlight
* Braking light
* Horn
* Unused
* Unused
