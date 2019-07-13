#!/usr/bin/env python3

import sys

class RegisterBlock():
    def __init__(self, file_path):
        # each entry is a tuple: ("reg name", offset, RW status)
        self.registers = []

        with open(file_path, "rt") as f:
            file_iter = iter(f)
            self.block_name = next(file_iter).strip()
            for line in file_iter:
                fields = line.split()
                offset = int(fields[0], 16)
                reg_name = fields[1]
                rw = fields[2]
                if rw == "RW1C":
                    rw = "RW"
                elif rw == "W1C":
                    rw = "WO"
                self.registers.append( (reg_name, offset, rw) )
            f.close()


    def __str__(self):
        lines = []
        lines.append("#[repr(C)]")
        lines.append("struct {} {{".format(self.block_name))

        last_offset=0x0
        reserved_block_num=0
        for reg_name, offset, rw in self.registers:
            num_words_between = int((offset - last_offset)/4) - 1
            if num_words_between > 0:
                lines.append(
                    "    Reserved{}: [RO<u32>; {}],".format(reserved_block_num, num_words_between)
                )
                reserved_block_num += 1
            lines.append("    pub {}: {}<u32>,".format(reg_name, rw))
            last_offset = offset

        lines.append('}')
        return '\n'.join(lines)

def main(file_path):
    reg_block = RegisterBlock(file_path)
    print(reg_block)

if __name__ == "__main__":
    if len(sys.argv) != 2:
        print("Must provide a file as an argument")
        sys.exit(1)
    main(sys.argv[1])
