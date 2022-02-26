import sys

class Reader:
    def __init__(self, data):
        self.data = data

    def read_bits(self, bits):
        result = 0

        for _ in range(bits):
            result <<= 1
            result |= self.data.pop(0)

        return result


class CountingReader:
    def __init__(self, reader):
        self.counted = 0
        self.reader = reader

    def read_bits(self, bits):
        self.counted += bits
        return self.reader.read_bits(bits)


class Literal:
    def __init__(self, version, value):
        self.version = version
        self.value = value

    def __str__(self):
        return f"Literal({self.version}, {self.value})\n"

    def version_sum(self):
        return self.version


class Operator:
    def __init__(self, version, sub_packets):
        self.version = version
        self.sub_packets = sub_packets

    def __str__(self):
        result = f"Operator({self.version}, {len(self.sub_packets)})\n"

        for sub_packet in self.sub_packets:
            for line in sub_packet.__str__().splitlines():
                result += "  " + line + "\n"

        return result

    def version_sum(self):
        return self.version + sum(map(lambda sub_packet: sub_packet.version_sum(), self.sub_packets))


def parse_literal_value(reader):
    result = 0

    while True:
        should_break = reader.read_bits(1) == 0

        result <<= 4
        result |= reader.read_bits(4)

        if should_break:
            break

    return result


def parse_packets_by_length(reader, length):
    reader = CountingReader(reader)
    sub_packets = []

    while reader.counted < length:
        sub_packets.append(parse(reader))

    return sub_packets


def parse_packets_by_number(reader, number):
    sub_packets = []

    for _ in range(number):
        sub_packets.append(parse(reader))

    return sub_packets


def parse(reader):
    version = reader.read_bits(3)
    type_id = reader.read_bits(3)

    if type_id == 4:
        value = parse_literal_value(reader)
        return Literal(version, value)
    else:
        length_type = reader.read_bits(1)
        sub_packets = None

        if length_type == 0:
            length = reader.read_bits(15)
            sub_packets = parse_packets_by_length(reader, length)
        else:
            number = reader.read_bits(11)
            sub_packets = parse_packets_by_number(reader, number)

        return Operator(version, sub_packets)


if __name__ == '__main__':
    if len(sys.argv) < 2:
        raise Exception("filename not speciifed, specify filename as first argument")

    filename = sys.argv[1]
    lines = None

    with open(filename) as f:
        line = f.read().strip()

    if not line:
        raise Exception("no lines found")

    hex_array = list(map(lambda c: int(c, base = 16), line))
    digits = []
    for value in hex_array:
        new_digit = []

        for _ in range(4):
            new_digit.append(value & 1)
            value >>= 1

        new_digit.reverse()
        digits.extend(new_digit)

    parsed = parse(Reader(digits))

    print(parsed)

    print("part 1: " + str(parsed.version_sum()))
    #print("part 2: " + str(lowest_risk(lines, 5)))
