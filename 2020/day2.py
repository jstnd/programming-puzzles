def occurrences(string, char):
    return len([i for i, letter in enumerate(string) if letter == char])


def main():
    valid = 0

    f = open("day2input.txt", "r")

    for ln in f:
        line = ln.replace(":", "").replace("-", " ").strip("\n").split(" ")
        if int(line[1]) >= occurrences(line[3], line[2]) >= int(line[0]):
            valid += 1

    print(valid)


if __name__ == "__main__":
    main()
