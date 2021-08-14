from os import major
import re

file = open("input", "r")

def put_bit(val, bit, pos):
    if(int(bit)):
        val |= (1 << pos)
    else:
        val &= ~(1 << pos)
    return val

def get_all_addrs(mask : str, addr : int):
    mask_list = []
    
    for i, c in enumerate(mask[::-1]):
        if c == "X":
            mask_list.append(i)
    floating_count = mask_list.__len__();

    addrs = []
    for i in range(0, pow(2,floating_count)):
        mask_ctr = bin(i)[2:].zfill(floating_count)
        addr_tmp = addr | int(mask.replace("X", "0"),2)
        for bit, pos in zip(mask_ctr, mask_list):
            addr_tmp = put_bit(addr_tmp, bit, pos)

        addrs.append(addr_tmp)
        print(".")

    return addrs

mem = {}
for line in file.readlines():
    if line.__contains__("mask"):
        mask = line.split(" ")[2].strip()
        continue

    match = re.search(r"mem\[([0-9]+)\] = ([0-9]+)", line)
    addrs = get_all_addrs(mask, int(match.group(1)))

    for addr in addrs:
        mem[addr] = int(match.group(2))

print(sum(mem.values()))

