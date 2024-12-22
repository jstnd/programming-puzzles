print(sum(len(ln.strip()) - len(eval(ln)) for ln in open("input08.txt")))  # part 1 - 1333
print(sum(len(ln.strip().replace("\\", r"\\").replace("\"", r"\"")) + 2 - len(ln.strip()) for ln in open("input08.txt")))  # part 2 - 2046
# print(sum(ln.count("\\") + ln.count('"') + 2 for ln in open("input08.txt")))  # alternate part 2 (saw in solutions thread and liked it)
