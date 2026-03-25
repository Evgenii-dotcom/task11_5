import os

FILE_PATH = "/data/shared.txt"


def write_data():
    try:
        with open(FILE_PATH, "a") as f:
            f.write("Python data\n")
    except IOError as e:
        raise RuntimeError(f"Write failed: {e}")


if __name__ == "__main__":
    write_data()