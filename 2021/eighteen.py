from itertools import combinations
with open("input", 'r') as fin:
    s = fin.read().strip()
    s = [[int(x) if x.isdigit() else x for x in v] for v in s.split('\n')]
def str_rep(a):
    return "".join([str(x) if type(x) is int else x for x in a])
def split(p, s):
    s = s[:p] + ['[', s[p] // 2, ',', (s[p]+1) // 2, ']'] + s[p+1:]
    return s

# p is position of ',' in the pair to be exploded
def explode(p, s):
    # find two numbers to be added and add them
    x,y = s[p-1], s[p+1]
    l = p - 2;
    while l > 0 and not type(s[l]) is int:
        l -= 1
    if l > 0:
        s[l] += x
    r = p + 2;
    while r < len(s) and not type(s[r]) is int:
        r += 1
    if r < len(s):
        s[r] += y

    #now destory the pair
    s = s[:p-2] + [0] + s[p+3:]
    return s

def add(a, b):
    s = ['['] + a + [','] + b + [']']
    done = False
    while not done:
        done = True
        nest, i = 0, 0
        # run explosions
        while i < len(s):
            nest += 1 if s[i] == '[' else -1 if s[i] == ']' else 0
            # too deep, explode and go
            if nest > 4 and type(s[i]) is int:
                done = False
                s = explode(i+1, s)
                nest = 0
                i = -1
            i += 1

        for i in range(len(s)):
            if type(s[i]) is int and s[i] > 9:
                s = split(i, s)
                done = False
                break
    return s
def magh(s):
    if type(s) is int:
        return s
    else:
        return 3*magh(s[0]) + 2*magh(s[1])
def mag(s):
    return magh(eval(str_rep(s)))

ans = -1
for x, y in combinations(s, 2):
    ans = max([ans, mag(add(x, y)), mag(add(y, x))])
print(ans)
