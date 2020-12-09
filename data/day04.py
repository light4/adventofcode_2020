from passport import Passport

part1 = 0  # number of valid passports for part 1
part2 = 0  # number of valid passports for part 2
with open("day04.txt") as day04:
    # Remember a set of lines (as passports can span multiple lines)
    # and only parse a passport as soon as an empty line is encountered
    currlines = ""
    for line in day04:
        if line.strip():
            currlines += line
        else:
            # Create a passport
            passport = Passport.from_lines(currlines)
            # Check the passport's validity
            part1 += passport.is_valid_day04_part1()
            part2 += passport.is_valid_day04_part2()
            # Empty the current lines to make place for the next passport
            currlines = ""

    # It could occur that currlines is still filled at this point
    # so repeat the process
    if currlines:
        passport = Passport.from_lines(currlines)
        part1 += passport.is_valid_day04_part1()
        part2 += passport.is_valid_day04_part2()

print(f"Part 1: {part1}")
print(f"Part 2: {part2}")
