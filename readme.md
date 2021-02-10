## about

NES emulator written in rust.

Following [this blog](https://bugzmanov.github.io/nes_ebook/chapter_1.html)

## notes

### architecture
- NES doesn't have an OS, games communicate directly with the hardware
    - machine language is the interface between the emulator and the ROM

### components
- CPU
- PPU
    - Picture Processing Unit
    - draw the game on the TV
- RAM
- APU
    - audio processing unit
- Cartridges
    - Character ROM
        - video graphics data
    - Program ROM
        - CPU instructions

### emulating CPU
- CPU has access to the Memory Map and CPU Registers

registers:
| Program Counter | Stack Pointer | Accumulator | Index Register X | Index Register Y | P |
------
| address of next instruction | top of the stack | result of math or logic | storage | storage | 8 bit register representing 7 status flags |



