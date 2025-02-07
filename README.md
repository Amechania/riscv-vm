I'll be adding a proper readme later.

This is a RISC-V based VM.

Currently supported:

 - Base Set RISCV32 instructions
 - MUL extension
 - Very basic view of register and memory pages
 - Simple MMU implementation

Future targets:

 - Atomic extension
 - Support for system calls.
 - Anything else to get a basic linux kernel running.
 - Serial port (duh).
 - Maybe 64bit option later? It requires a ton of work and intructions.
