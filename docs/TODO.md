# TODO

## Known bugs
- [ ] Mixed mode push ((ismixed<<1) | isadl) to stack. I haven't implemented the ismixed part
- [ ] c8 joystick gpio mutex contention on windows -- probably use atomics instead
- [ ] ez80f92 PRT gpio-b pin 1 source (vblank) not implemented

## VDP flaws and omissions
- [ ] ESP32 RAM is unlimited. Figure out ESP32 PSRAM accounting issues (C++ custom allocators)
- [ ] vdp audio sample rate setting not implemented
- [ ] VDP key repeat rate settings are ignored, and host system key repeat is used. But maybe this is good.
- [ ] PAUSE key is subject to key repeat, which it is not on a real agon.
- [ ] Copper effects not implemented
- [ ] Hardware sprites not implemented

## Feature ideas
- [ ] symbol support in debugger
- [ ] uart1 host serial link (some work done but untested)
- [ ] gdbserver support?
