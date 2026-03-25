import os
from app import write_data


def test_write(tmp_path):
    file_path = tmp_path / "test.txt"
    os.environ["FILE_PATH"] = str(file_path)

    write_data()
    assert file_path.exists()