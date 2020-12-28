def run(fn, nl):  # todo: remove turn history for speedup
    sp = {n: [-1, i + 1] for i, n in enumerate(nl)}
    for t in range(len(nl) + 1, fn + 1):
        nx = sp[nl[-1]][1] - sp[nl[-1]][0] if sp[nl[-1]][0] != -1 else 0
        if nx in sp:
            sp[nx][0], sp[nx][1] = sp[nx][1], t
        else:
            sp[nx] = [-1, t]
        nl.append(nx)
    return nl[-1]


nums = [0, 14, 1, 3, 7, 9]
print(run(2020, nums))
print(run(30000000, nums))
