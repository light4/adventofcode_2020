import re


def check_byr(s):
    return 1920 <= int(s) <= 2002


def check_iyr(s):
    return 2010 <= int(s) <= 2020


def check_eyr(s):
    return 2020 <= int(s) <= 2030


def check_hgt(s):
    if not (s.endswith('cm') or s.endswith('in')):
        return False
    n = s[:-2]
    if s.endswith('cm'):
        return 150 <= int(n) <= 193
    if s.endswith('in'):
        return 59 <= int(n) <= 76


def check_hcl(s):
    if re.match(r'^#[0-9a-f]{6}$', s):
        return True
    else:
        return False


def check_ecl(s):
    return s in ['amb', 'blu', 'brn', 'gry', 'grn', 'hzl', 'oth']


def check_pid(s):
    return s.isdigit() and len(s) == 9


check_funcs = {
    'byr': check_byr,
    'iyr': check_iyr,
    'eyr': check_eyr,
    'hgt': check_hgt,
    'hcl': check_hcl,
    'ecl': check_ecl,
    'pid': check_pid,
}


def test():
    assert check_byr('2002')
    assert not check_byr('2003')

    assert check_hgt('60in')
    assert check_hgt('190cm')
    assert not check_hgt('190in')
    assert not check_hgt('190')

    assert check_hcl('#123abc')
    assert not check_hcl('#123abz')
    assert not check_hcl('123abc')

    assert check_ecl('brn')
    assert not check_ecl('wat')

    assert check_pid('000000001')
    assert not check_pid('0123456789')


def get_need_check_items(data):
    valid_keys = set(check_funcs.keys())

    valid_passport_items = []
    for passport in data:
        fields = [i for i in re.split(r'\s', passport) if i]
        passport_item = {i.split(':')[0]: i.split(':')[1] for i in fields}
        if valid_keys.intersection(passport_item.keys()) == valid_keys:
            valid_passport_items.append(passport_item)
    return valid_passport_items


def main():
    with open('day04.txt') as f:
        data = re.split('\n\n', f.read())

    need_check_items = get_need_check_items(data)
    count = 0
    for i in need_check_items:
        if all(v(i[k]) for k, v in check_funcs.items()):
            count += 1
    print(count)


if __name__ == "__main__":
    main()
