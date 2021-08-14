import re

file = open("input", "r")

def parse_mask(mask_str : str):
    mask = {}
    mask_str = (mask_str.replace("X", "2")).strip()

    for i, c in enumerate(mask_str[::-1]):
        if int(c) == 0 or int(c) == 1:
            mask[i] = int(c)

    return mask 

def mask_val(mask : dict, val : int):
    for key in mask:
        if(mask[key]):
            val |= (1 << key)
        else:
            val &= ~(1 << key)
    return val

mem = {}

for line in file.readlines():
    if line.__contains__("mask"):
        mask = {}
        mask = parse_mask(line.split(" ")[2])
        continue
    match = re.search(r"mem\[([0-9]+)\] = ([0-9]+)", line)
    new_val = mask_val(mask, int(match.group(2)))
    mem[match.group(1)] = new_val

print(sum(mem.values()))
