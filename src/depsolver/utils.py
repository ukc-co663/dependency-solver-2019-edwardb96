def zip_with(f, left, right):
    return (f(x, y) for (x, y) in zip(left, right))

def adjacent_pairs(f, list):
    fst = iter(list)
    snd = iter(list)
    next(snd)
    for s in snd:
        yield f(next(fst), s)
