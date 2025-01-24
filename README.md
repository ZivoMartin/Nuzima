# **Nuzima Assembler**

The Nuzima Assembler is a lightweight assembly tool designed for generating Nuzima bytecode. This assembler allows you to write and compile low-level instructions that can be executed by the Zivojinovic Virtual Machine (ZVM).
Features

    Comprehensive Instruction Set:
    The Nuzima Assembler supports a variety of instructions, enabling efficient and versatile bytecode generation.
    Seamless Integration with ZVM:
    Assembled bytecode files are fully compatible with ZVM for execution.

**Supported Instructions**

The following instruction types are supported by the Nuzima Assembler:
Arithmetic Operations

    ADD: Add two values.
    MUL: Multiply two values.
    SUB: Subtract one value from another.
    DIV: Divide one value by another.
    MOD: Compute the remainder of division.
    NEG: Negate a value.

**Bitwise Operations**

    SHL: Shift bits to the left.
    SHR: Shift bits to the right.
    AND: Perform a bitwise AND operation.
    OR: Perform a bitwise OR operation.
    XOR: Perform a bitwise XOR operation.
    NOT: Perform a bitwise NOT operation.

**Control Flow**

    JMP: JE, JH, JL, JLE, JHE, JMP
    CALL: Call a subroutine.
    RET: Return from a subroutine.
    CMP: Compare two values for equality.

**Stack Operations**

    PUSH: Push a value onto the stack.
    POP: Pop a value from the stack.
    DUP: Duplicate the top value on the stack.
    SWAP: Swap the top two values on the stack.

Memory Operations

    MOV: Move data between registers or memory.
    READ: Read data from memory.
    WRITE: Write data to memory.

System and Debugging

    INT: Trigger an interrupt.
    TRACE: Output debug information.
    HALT: Halt the execution of the program.

Miscellaneous

    CLEAR: Clear specific registers or memory.

Usage

    Write your assembly code using the supported instructions in a text file (e.g., program.asm).
    Use the Nuzima Assembler to compile the file into Nuzima bytecode:

    ./nuzima-assembler program.asm -o program.nbc

    Load the bytecode file (program.nbc) into the ZVM to execute.

Roadmap

The Nuzima Assembler is actively being developed, with plans for the following enhancements:

    Improved debugging tools.
    Enhanced error reporting during assembly.
    Expanded instruction set.

Contributing

Contributions are welcome! Feel free to submit issues, feature requests, or pull requests to help improve the Nuzima Assembler.

Let me know if you'd like to add anything specific to this README!
