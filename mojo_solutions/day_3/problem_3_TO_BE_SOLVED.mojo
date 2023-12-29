def test():
    var f: PythonObject = [(1, 2), 2]

    for i in f:
        print(i)


fn has_adjacent_symbol(
    lines: DynamicVector[String],
    line_length: Int,
    number_of_lines: Int,
    x_pos: Int,
    y_pos: Int,
) raises -> Bool:
    var neighbours = [
        (-1, -1),
        (0, -1),
        (-1, 1),
        (-1, 0),
        (0, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ]

    for pair in neighbours:
        let x = int(x_pos + pair[0])
        let y = int(y_pos + pair[1])
        # Initally check if checking around the character is within bounds
        if (
            x_pos >= 0
            and x_pos <= line_length - 1
            and y_pos <= number_of_lines - 1
            and y_pos >= 0
        ):
            return not isdigit(ord(lines[x][y])) and lines[x][y] != "."

    return False


fn part_a(lines: DynamicVector[String]) raises:
    # Potential solution
    # Identify numbers
    # Out of the numbers, identify which ones have symbols adjacent to themo
    var sum = 0
    for l in range(len(lines)):
        let line = lines[l]
        let number_of_lines = len(lines)
        var number: String = ""
        var add_number = False
        for c in range(len(line)):
            let character = line[c]
            let line_length = len(line)
            if isdigit(ord(character)):
                number += character
            else:
                break
            # Scan around the character to see if there are adjacent symbols
            if has_adjacent_symbol(
                lines,
                number_of_lines,
                line_length,
                c,
                l,
            ):
                print("Has adjacent symbol!")


fn part_b(lines: DynamicVector[String]):
    pass


fn main() raises:
    test()
    with open("../problem_3.txt", "r") as f:
        let data = f.read()
        let lines = data.split("\n")
        part_a(lines)
