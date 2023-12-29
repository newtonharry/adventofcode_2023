from python import Python


fn part_a(lines: DynamicVector[String]) raises:
    var first: String = ""
    var second: String = ""
    var sum = 0
    for i in range(lines.__len__()):
        let line: String = lines[i]
        # Go from left to right
        for char in range(line.__len__()):
            if isdigit(ord(line[char])):
                first = line[char]
                break

        # Go from right to left
        for char in range(line.__len__(), -1, -1):
            if isdigit(ord(line[char])):
                second = line[char]
                break

        sum += atol(first + second)

    print(sum)


fn is_number(vec: String) -> Int:
    var numbers: PythonObject = [
        "one",
        "two",
        "three",
        "four",
        "five",
        "six",
        "seven",
        "eight",
        "nine",
    ]
    for num in numbers:
        if vec.find(str(num)) > -1:
            return len(str(num))
    return -1


fn part_b(lines: DynamicVector[String]) raises:
    var first: String = ""
    var second: String = ""
    var sum = 0
    let word_to_char = Python.dict()
    word_to_char["one"] = "1"
    word_to_char["two"] = "2"
    word_to_char["three"] = "3"
    word_to_char["four"] = "4"
    word_to_char["five"] = "5"
    word_to_char["six"] = "6"
    word_to_char["seven"] = "7"
    word_to_char["eight"] = "8"
    word_to_char["nine"] = "9"
    for i in range(lines.__len__()):
        let line: String = lines[i]
        # Go from left to right
        for char in range(line.__len__()):
            if isdigit(ord(line[char])):
                first = line[char]

            let num_str_len = is_number(line[:char])
            if num_str_len > -1:
                first = str(word_to_char[line[char - num_str_len : char]])

            if first != "":
                break

        # Go from right to left
        for char in range(line.__len__(), -1, -1):
            if isdigit(ord(line[char])):
                second = line[char]

            let num_str_len = is_number(line[char:])
            if num_str_len > -1:
                second = str(word_to_char[line[char : char + num_str_len]])

            if second != "":
                break

        sum += atol(first + second)
        first = ""
        second = ""

    print(sum)


fn main() raises:
    # with open("../problem_1_part_a.txt", "r") as f:
    #     var data = f.read()
    #     var lines = data.split("\n")
    #     part_a(lines)

    with open("../problem_1_part_b.txt", "r") as f:
        let data = f.read()
        let lines = data.split("\n")
        part_b(lines)

    # var part_b_data = open("../problem_1_part_b.txt", "r")
