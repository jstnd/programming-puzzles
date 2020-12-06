moves = {"^": (0, 1), "v": (0, -1), ">": (1, 0), "<": (-1, 0)}


def deliver(ds):
    pos = (0, 0)
    return set((pos := tuple(map(sum, zip(pos, moves[d])))) for d in ds) | {(0, 0)}


dirs = open("input03.txt").read()

print(len(deliver(dirs)))  # part 1 - 2572
print(len(deliver(dirs[::2]) | deliver(dirs[1::2])))  # part 2 - 2631
