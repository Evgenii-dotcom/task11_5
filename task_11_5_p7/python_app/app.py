import rust_ext


def calculate():
    return rust_ext.sum_as_string(2, 3)


if __name__ == "__main__":
    print(calculate())