
def is_safe(n):
    g = all(a < b for a,b in zip(n, n[1:])) or all(a > b for a,b in zip(n, n[1:]))
    if g:
        n.sort()
        if any(a for a,b in zip(n, n[1:]) if b-a > 3 or b-a < 1):
            g = False
    return g 

if __name__ == '__main__':
    # with open("input.txt") as f:
    with open("example.txt") as f:
        data = f.read().strip()

    answer = 0

    for l in data.split('\n'):
        n = [int(x) for x in l.split()]
        if is_safe(n) or any(is_safe(n[:i]+n[i+1:]) for i in range(len(n))):
            answer += 1

    print(answer)

