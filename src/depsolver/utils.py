def zip_with(f, *args):
    return (f(*a) for a in zip(*args))

def adjacent_pairs(f, list):
    fst = iter(list)
    snd = iter(list)
    next(snd)
    for s in snd:
        yield f(next(fst), s)
