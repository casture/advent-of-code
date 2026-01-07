
from collections import Counter


if __name__ == '__main__':
    with open("input.txt") as f:
        data = f.read().strip()

    left, right = [], []
    for l in data.split('\n'):
        n = l.split()
        left.append(int(n[0]))
        right.append(int(n[1]))

    left.sort()
    right.sort()

    a = 0
    s = sum([abs(l-r) for l, r in zip(left, right)])

    print(s)

    right = Counter(right)
    a = 0
    for l in left:
        a += l * right[l]

    print(a)
