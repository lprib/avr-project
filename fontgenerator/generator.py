#!/usr/bin/python3

# Converts a font.txt file into a .c byte array

import sys

# file to read
FONT_FILENAME = "font.txt"
# characters and order to expect in the font file
CHARACTERS = " abcdefghijklmnopqrstuvwxyz0123456789"
# character denoting an "on" pixel
ON_CHARACTER = "0"
# character denoting an "off" pixel
OFF_CHARACTER = "."
# used to indent when emitting c code
INDENT = "  "

l = open(FONT_FILENAME).readlines();
print("unsigned char font[] = {")

for index, char in enumerate(CHARACTERS):
    print(INDENT + "// {}".format(char))
    for i in range(0,5):
        print(INDENT + "0b", end="")
        for j in range(0,3):
            pixel = l[index * 6 + 1 + i][j]
            if pixel == OFF_CHARACTER:
                print("0", end="")
            elif pixel == ON_CHARACTER:
                print("1", end="")
            else:
                sys.exit("Expected {} or {} in characted definition".format(ON_CHARACTER, OFF_CHARACTER))
        if char == CHARACTERS[-1] and i == 4:
            print()
        else:
            print(",")
print("};")