import os
import random as rand


def generate_file(path, approx_size, signature=None, random=False, chunksize=1 << 16):
    def gen_chunk(n):
        return os.urandom(n) if random else b"A" * n

    with open(path, "wb") as f:
        size = 0

        if approx_size >= chunksize:
            for i in range(approx_size // chunksize + 1):
                size += f.write(gen_chunk(chunksize))
        else:
            size += f.write(gen_chunk(approx_size))

        if signature is not None:
            signature_pos = rand.randint(0, size - 1)
            print("File {} will have signature at {}".format(path, signature_pos))
            f.seek(signature_pos)
            f.write(signature)


def generate_tree(path, depth,
                  min_file_size, max_file_size,
                  min_files, max_files,
                  min_dirs, max_dirs,
                  signature, signature_chance):
    os.makedirs(path, exist_ok=True)

    for i in range(rand.randint(min_files, max_files)):
        generate_file(os.path.join(path, "f_{}.file".format(i)),
                      approx_size=rand.randint(min_file_size, max_file_size),
                      signature=signature if rand.random() < signature_chance else None,
                      random=True)

    if depth <= 0:
        return

    for i in range(rand.randint(min_dirs, max_dirs)):
        generate_tree(os.path.join(path, "d_{}.dir".format(i)), depth - 1,
                      min_file_size, max_file_size,
                      min_files, max_files,
                      min_dirs, max_dirs,
                      signature, signature_chance)


if __name__ == '__main__':
    signature = "small hidden text 123 456".encode(encoding="utf8")
    generate_large_outlier = True

    generate_tree(
        "test-dir-tree", 2,
        0, (1 << 20) * 64,  # 64mb max
        1, 20, 1, 3,
        signature, signature_chance=0.5
    )

    if generate_large_outlier:
        generate_tree(
            "test-dir-tree", 0,
            1 << 30, 2 * (1 << 30),
            1, 2, 0, 0,
            signature, signature_chance=0.5
        )

    with open("signature.file", "wb") as f:
        f.write(signature)
