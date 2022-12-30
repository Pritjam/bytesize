# bytesize

## General Info
This is a 16-bit CPU architecture. There are 8 general-purpose registers, all 16 bits wide, and a 16-bit instruction pointer. The CPU can address 128k of ram as 64Kx16. There is also support for IO operations. The instruction set is detailed more in the ISA section, but in general, the ISA is designed to be a simplified version of ARM64, with much less features and a little x86 inspiration. 

Any GPR can be used in an ALU operation or data movement operation. The first 4, designated %ax, %bx, %cx, and %dx, are intended for general-purpose data usage. The other 4, %si, %di, %bp, and %sp, are intended for more specific use something along the lines of x86’s registers: Source Index, Destination Index, Base Pointer, and Stack Pointer. The Stack Pointer of this CPU is %sp, and the CALL and RET instructions use that as the stack pointer to push/pop the return address. The PUSH and POP instructions also use %sp as the stack pointer, but since those assemble into pre- and post-indexed LOAD and STORE operations respectively, any GPR could be used as a “stack pointer” at any time. 

An instruction cycle is made of 5 machine cycles, and each machine cycle performs a different subtask of one instruction. This follows the 5-stage Fetch, Decode, Execute, Memory, Writeback paradigm. The CPU supports a dual register writeback, which is required in the case of an instruction that reads from memory and modifies a pointer, such as POP or a pre/post-indexed LOAD.

This CPU will be implemented in software as a Rust project, very similar to SE-Lab (System Emulator lab, found in UTCS’s Computer Architecture class). This project should be a good introduction to Rust as well as good practice with converting existing C projects into Rust.

As for physical implementation, this CPU will eventually be implemented using a Xilinx Artix-7 FPGA on the BASYS 3 trainer board. This FPGA should have more than enough capacity for this CPU, as well as enough on-board RAM (225KB) to support it. Eventually, we should be able to build up an entire computer on the Basys, as it incorporates a VGA port, USB HID port (presented internally as a PS/2 port), and even peripheral slots (for a possible disk/IO interface).

## TODO
- List of valid instructions (not opcodes, instructions)
    - break up ALUop into each sub-instr
- Write an instruction page template
- Standardize operand format in line with A64 following 1.0 revision
- Floating point? Could fall in the 00011 block?
- Remove Versioning section or move it to a separate doc
- Set up Project page on personal site

## Versioning
### 1.0
- Basic instructions
- No IO
- Only Base + Offset indexing
- No vectorization
- Sequential implementation
- No shadow registers
- No calling convention standard
- No peripherals
- No bus
### 1.0.1
- Very basic assembler in Python, which will evolve with future versions
### 1.1
- Adds shadow registers
- Adds XCHNG instruction
### 1.2
- Adds exceptions and interrupts, a simple variable poll at this time
### 1.3
- Adds CPU and memory save state
- Adds vectorization
### 1.4 (OPTIONAL)
- Pipelined implementation, with forwarding and hazard control
### 2.0
- System Bus Architecture
- Memory on System Bus
- IO support
### 2.1
- Hard Disk (IDE or something) support
- Rework interrupts to use threads and channels
- PIC

## ISA
Each instruction is 2 bytes. These instructions can contain several data fields, which are specified in that instruction’s page. This format is intended to be similar to the ARMv8 specification in the ARM Architecture Reference Manual.
### Indexing Syntax
In this ISA, many operands of assembly instructions involve memory or registers. The syntax for these are listed below. In these syntaxes, curly braces `{}` indicate an optional element, while parentheses `()` indicate a required element. Square brackets `[]` do not have a syntactic function--if they appear in syntax, that means they are part of the syntax.
- `<R(d|s|t)>` indicates a register operand. The letter used helps identify the purpose of that operand. “d” indicates a destination register, while “s” indicates a source register. “t” indicates a “transfer” register, which is the register to be transferred to/from in a memory operation.
- #imm indicates an integer literal. The specification for the instruction in which this operand appears details exactly how many bits the operand is allowed. Integer literals can be written without an initial character, in which case they are interpreted in decimal, or they can be written starting with a dollar sign, in which case they are interpreted in hexadecimal. For example, the literals 10 and $a are equivalent. Hexadecimal literals can be capitalized or in lowercase.
- Memory operands are specified as `[mem]`. There are 3 forms this memory operand can take:
    - Base + Offset: `[<Rs>{, #imm}]`
    - Pre-Indexed: `[<Rs>, #imm]!`
    - Post-Indexed: `[<Rs>], #imm`
- Labels (jump targets) are denoted by .LABEL. 

### Registers
There are 8 general-purpose registers in this architecture. These are labeled:
%ax, %bx, %cx, %dx, %bp, %si, %di, %sp

There are also a series of “shadow” registers that can be swapped out for their corresponding “actual” register. These shadow registers are indicated with an apostrophe after their name. They are:
%ax’, %bx’, %cx’, %dx’, and %FLAGS'

The EXX instruction is used to swap registers %ax, %bx, %cx, %dx. This instruction does not specifically swap in/out the shadow registers; instead it simply toggles the currently referenced register.

Similarly, the EXF instruction is used to swap the  %FLAGS register. Again, it does not specifically swap in/out the shadow flags register; instead it simply toggles the active flags register.

These shadow registers are intended to be used with interrupt handling, as they preserve user data in the non-shadow registers and user flags in the non-shadow flags register. However, on systems without interrupts (or if interrupts are temporarily disabled), the shadow registers could be used for very fast leaf subroutines, bypassing the need to save the first 4 registers on the stack.
