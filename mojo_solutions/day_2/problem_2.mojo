@value
struct GamePartA(CollectionElement):
    var game_id: Int
    var impossible: Bool


fn part_a(lines: DynamicVector[String]) raises:
    var games = DynamicVector[GamePartA]()
    var sum = 0
    for line in range(len(lines)):
        var impossible = False
        let game = lines[line].split(":")
        let sets = game[1].strip().split(";")
        for set in range(len(sets)):
            let cubes = sets[set].strip().split(",")
            for cube in range(len(cubes)):
                let cube_info = cubes[cube].strip().split(" ")
                let amount = atol(cube_info[0])
                let colour = cube_info[1]
                if (
                    (colour == "red" and amount > 12)
                    or (colour == "green" and amount > 13)
                    or (colour == "blue" and amount > 14)
                ):
                    impossible = True
                    break
            if impossible == True:
                break
        games.append(GamePartA(line + 1, impossible))
        impossible = False

    for game in range(len(games)):
        if not games[game].impossible:
            sum += games[game].game_id

    print(sum)


@value
struct GamePartB(CollectionElement):
    var min_red: Int
    var min_green: Int
    var min_blue: Int

    fn power(self) -> Int:
        return self.min_red * self.min_green * self.min_blue


fn part_b(lines: DynamicVector[String]) raises:
    var games = DynamicVector[GamePartB]()
    var sum = 0
    for line in range(len(lines)):
        let game = lines[line].split(":")
        var game_obj = GamePartB(0, 0, 0)
        let sets = game[1].strip().split(";")
        for set in range(len(sets)):
            let cubes = sets[set].strip().split(",")
            for cube in range(len(cubes)):
                let cube_info = cubes[cube].strip().split(" ")
                let amount = atol(cube_info[0])
                let colour = cube_info[1]
                if colour == "red" and amount > game_obj.min_red:
                    game_obj.min_red = amount
                elif colour == "green" and amount > game_obj.min_green:
                    game_obj.min_green = amount
                elif colour == "blue" and amount > game_obj.min_blue:
                    game_obj.min_blue = amount

        games.append(game_obj)

    for game in range(len(games)):
        sum += games[game].power()

    print(sum)


fn main() raises:
    with open("../problem_2_part_a.txt", "r") as f:
        let data = f.read()
        let lines = data.split("\n")
        part_a(lines)

    with open("../problem_2_part_b.txt", "r") as f:
        let data = f.read()
        let lines = data.split("\n")
        part_b(lines)
