from collections import Counter


RU_LETTERS = set("абвгдеёжзиклмнопрстуфхцчшщьыъэюя")


def slide_filtered(s, fn, w):
    for start in range(len(s) - w + 1):
        window = s[start:start+w]
        if all(map(fn, window)):
            yield window


def frequencies(text, f, win_size):
    counts = Counter(slide_filtered(text, f, win_size))
    total = sum(counts.values())
    return list(map(lambda p: (p[0], p[1] / total), counts.most_common()))


if __name__ == '__main__':
    import sys
    import time
    path = sys.argv[1]
    window_size = int(sys.argv[2])

    t = time.perf_counter()
    with open(path) as text:
        for el, freq in frequencies(text.read().lower(), lambda c: c in RU_LETTERS, window_size):
            print("{} - {:.6f}".format(el, freq))
    print("Done in {:.3f} s".format(time.perf_counter() - t))
