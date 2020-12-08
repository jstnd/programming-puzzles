import re


def part1(ports):
    valid = 0

    for p in ports:
        current = p.split()
        valid += len(current) == 8 or (len(current) == 7 and not [s for s in current if s.startswith("cid")])

    return valid


def part2(ports):
    valid = 0

    for p in ports:
        current = p.split()
        if len(current) == 8 or (len(current) == 7 and not [s for s in current if s.startswith("cid")]):
            valid += is_valid_passport(current)

    return valid


def is_valid_passport(port):
    eye_colors = {"amb", "blu", "brn", "gry", "grn", "hzl", "oth"}

    for e in port:
        field = e.split(":")
        if field[0] == "byr":
            if not (len(field[1]) == 4 and 1920 <= int(field[1]) <= 2002):
                return False
        elif field[0] == "iyr":
            if not (len(field[1]) == 4 and 2010 <= int(field[1]) <= 2020):
                return False
        elif field[0] == "eyr":
            if not (len(field[1]) == 4 and 2020 <= int(field[1]) <= 2030):
                return False
        elif field[0] == "hgt":
            if not (((field[1])[-2:] == "cm" and 150 <= int((field[1])[:-2]) <= 193)
                    or ((field[1])[-2:] == "in" and 59 <= int((field[1])[:-2]) <= 76)):
                return False
        elif field[0] == "hcl":
            if not re.match("^#[\d|(a-f)]{6}$", field[1]):
                return False
        elif field[0] == "ecl":
            if not field[1] in eye_colors:
                return False
        elif field[0] == "pid":
            if not re.match("^\d{9}$", field[1]):
                return False

    return True


passports = [p.replace("\n", " ") for p in open("input04.txt").read().split("\n\n")]
print(part1(passports))  # 256
print(part2(passports))  # 198
