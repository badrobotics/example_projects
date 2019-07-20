# AWranovsky Test
## Summary
This is my project for testing new things out in. Right now it's just the bare
minimum to get the microcontroller to run at its maximum operating frequency
and to get an LED blinking.

## Running the program
1. Navigate to the project root.
2. Run `openocd` here.
3. In another terminal, run `arm-none-eabi-gdb -q <path to binary>`.
4. In gdb, run `target extended-remote :3333`.
5. Then run `load` in gdb.
