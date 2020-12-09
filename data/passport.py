import re


def check_hgt(s):
    if not (s.endswith('cm') or s.endswith('in')):
        return False
    n = s[:-2]
    if s.endswith('cm'):
        return 150 <= int(n) <= 193
    if s.endswith('in'):
        return 59 <= int(n) <= 76


def check_hcl(s):
    return not not re.match(r'#[1-9a-f]{6}', s)


class Passport:
    # required password fields
    reqfields = ("byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid")
    # optional password fields (not used yet)
    optfields = ("cid", )
    # regexes for password fields
    field_rx = {
        "hgt": re.compile(r"^([0-9]{2,3})(cm|in)$"),
        "hcl": re.compile(r"^#[0-9a-f]{6}$"),
        "ecl": re.compile(r"^(amb|blu|brn|gry|grn|hzl|oth)$"),
        "pid": re.compile(r"^([0-9]{9})$")
    }

    def __init__(self, **kwargs):
        for kwarg, value in kwargs.items():
            setattr(self, kwarg, value)

    @classmethod
    def from_lines(cls, lines: str):
        """Read lines and return a Passport object"""
        kwargs = {}
        for field in lines.split():
            key, val = field.split(":")
            kwargs[key] = val
        return cls(**kwargs)

    def is_valid_day04_part1(self):
        """Checks validity of a passport"""
        return all(hasattr(self, attr) for attr in self.reqfields)

    def is_valid_day04_part2(self):
        """Checks validity of a passport"""
        if not self.is_valid_day04_part1():
            # logger.debug("Invalid: not all required fields present")
            return False
        # birth year
        if int(self.byr) not in range(1920, 2003):
            # logger.debug(f"Invalid birth year {self.byr}")
            return False
        # issue year
        if int(self.iyr) not in range(2010, 2021):
            # logger.debug(f"Invalid issue year {self.iyr}")
            return False
        # expiration year
        if int(self.eyr) not in range(2020, 2031):
            # logger.debug(f"Invalid expiration year {self.eyr}")
            return False
        # height
        hgtmatch = self.field_rx["hgt"].match(self.hgt)
        if not hgtmatch:
            # logger.debug(f"Invalid height: {self.hgt}")
            return False
        value, unit = hgtmatch.groups()
        if unit == "cm" and int(value) not in range(150, 194):
            # logger.debug(f"Too short/tall: {self.hgt}")
            return False
        elif unit == "in" and int(value) not in range(59, 77):
            # logger.debug(f"Too short/tall: {self.hgt}")
            return False
        elif unit not in ("cm", "in"):
            # logger.debug(f"Invalid unit: {self.hgt}")
            return False

        # And finally, some simple matches
        for field in ("hcl", "ecl", "pid"):
            value = getattr(self, field)
            fieldmatch = self.field_rx[field].match(value)
            if not fieldmatch:
                # logger.debug(f"Invalid {field}: {value}")
                return False

        # logger.success("Valid!")
        return True
