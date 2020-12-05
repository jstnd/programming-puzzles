import hashlib


def find(msg, start):
    count = 0
    while True:
        count += 1
        if hashlib.md5((msg + str(count)).encode()).hexdigest()[:len(start)] == start:
            break

    return count


print(find("bgvyzdsv", "00000"))   # part 1 - 254575
print(find("bgvyzdsv", "000000"))  # part 2 - 1038736
