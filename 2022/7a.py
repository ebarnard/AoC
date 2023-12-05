def run(input_path):
    input = open(input_path, "r")

    dir_stack = []
    dir_sizes = []

    def finish_dir():
        dir_name, size = dir_stack.pop()
        print(dir_name, "has size", size)
        if len(dir_stack) > 0:
            add_file(size)
        dir_sizes.append(size)

    def add_file(size):
        dir_name, total_size = dir_stack.pop()
        dir_stack.append((dir_name, total_size + size))

    for line in input.readlines():
        line = line.strip()

        if line.startswith("$ cd "):
            dir_name = line[5:]
            if dir_name == "..":
                finish_dir()
            else:
                if dir_stack:
                    dir_path = dir_stack[-1][0] + "/" + dir_name
                else:
                    dir_path = dir_name
                dir_stack.append((dir_path, 0))

        elif line.startswith("$ ls"):
            pass

        elif line.startswith("dir "):
            pass

        else:
            size, _ = line.split(" ")
            add_file(int(size))

    while dir_stack:
        finish_dir()

    return sum((s for s in dir_sizes if s <= 100000))


if __name__ == "__main__":
    assert run("7-test.txt") == 95437
    assert run("7-real.txt") == 1141028
