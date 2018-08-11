# Driver Information System (DIS)

All information here is just a rough idea and subject to complete change.

## Purpose

The DIS is responsible for collecting all relevant data from all other systems and display the
required information to the driver via a 7 inch screen. In addition to this, the DIS also collects
all the telemetry data and stores them for later use.

## System

The DIS runs on a raspberry pi with stripped down ArchLinux ARM distro. With this, I hope to reach a boot-time
below 5s. The software stack will be very bare bone and contain only the required minimum to interface with the
other systems and internal peripeheral like the TFT, SD-Card, evtl. bluetooth, etc. The driver can interact with 
the DIS via a resistive touchscreen. 

## Graphics Engine

The rendering will be done via openGLES. The display will show relevant information to the driver

Information to display
- Current speed
- Current power consumption
- Estimated range
- Signal status (Turn Signal)
- Diagnosis information


## Telemtry

Telemtry is gathered from:

- APC
- Motor controller
- Battery management