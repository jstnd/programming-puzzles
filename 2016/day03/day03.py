inp = [list(map(int, ln.strip().split())) for ln in open("input03.txt")]
print(sum(a + b > c and a + c > b and b + c > a for a, b, c in inp))  # part 1 - 993
print(sum(sum(inp[i + a][t] + inp[i + b][t] > inp[i + c][t] for a, b, c in [(0, 1, 2), (0, 2, 1), (1, 2, 0)]) == 3 for i in range(0, len(inp), 3) for t in range(len(inp[i]))))  # part 2 - 1849
