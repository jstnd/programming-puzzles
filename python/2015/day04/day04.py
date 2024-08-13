import hashlib


def find(msg, start):
    count = 1
    while hashlib.md5((msg + str(count)).encode()).hexdigest()[:len(start)] != start:
        count += 1

    return count


print(find("bgvyzdsv", "00000"))   # part 1 - 254575
print(find("bgvyzdsv", "000000"))  # part 2 - 1038736
