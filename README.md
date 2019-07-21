# AWranovsky Test
## Summary
This is my project for testing new things out in. Right now it's just the bare
minimum to get the microcontroller to run at its maximum operating frequency
and to get an LED blinking. I'll probably end up renaming this repository
later.

## Running the program
1. Navigate to the project root.
2. Run `openocd`.
3. Run `arm-none-eabi-gdb`.
4. ???
5. Profit

## GDB tips
* Pass `-tui` to gdb on the command line, and it'll use the text gui, which is
  much easier to use than the default.
* No arguments need to be passed to gdb or openocd because I've provided
  configuration files for them in the project root.
* To autoload the `.gdbinit` file, you may need to append this line to your
  `~/.gdbinit` file, substituting the path name as necessary:
  `add-auto-load-safe-path /path/to/project/.gdbinit`
