import re

REGEX = {
    "byr": "^(19[2-9]\d|200[0-2])$",  # 1920 - 2002
    "iyr": "^20(1\d|20)$",  # 2010 - 2020
    "eyr": "^20(2\d|30)$",  # 2020 - 2030
    "hgt": "^(1([5-8]\d|9[0-3])cm|(59|6\d|7[0-6])in)$",  # 150-193 in | 59-76 cm
    "hcl": "^#[\d|(a-f)]{6}$",  # # followed by six characters 0-9 or a-f
    "ecl": "^(amb|blu|brn|gry|grn|hzl|oth)$",
    "pid": "^\d{9}$"  # nine digit number
}

passports = [p.replace("\n", " ").split() for p in open("input04.txt").read().split("\n\n")]

print(sum(sum(f[:3] != "cid" for f in p) == 7 for p in passports))  # part 1 - 256
print(sum(sum(1 if re.match(REGEX[f[:3]], f[4:]) else 0 for f in p if f[:3] != "cid") == 7 for p in passports))  # part 2 -  198
