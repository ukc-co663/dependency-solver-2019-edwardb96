def zip_with(f, left, right):
    return (f(x, y) for (x, y) in zip(left, right))
