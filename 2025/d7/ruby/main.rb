require "debug"

def parse(input)
  lines = input.lines.map do |line|
    line.strip.chars.to_a
  end

  start = lines.shift.find_index('S')

  [start, lines]
end

def part1(start, lines)
  tachs = Set[start]
  splits = 0

  lines.each do |line|
    new_tachs = Set.new

    tachs.each do |tach|
      if line[tach] == '^'
        new_tachs << tach + 1
        new_tachs << tach - 1
        splits += 1
      else
        new_tachs << tach
      end
    end

    tachs = new_tachs
  end

  splits
end

def part2(start, lines)
  tachs = Array.new(lines[0].length, 0)
  tachs[start] = 1

  lines.each do |line|
    new_tachs = Array.new(tachs.length, 0)

    line.each.with_index do |loc, idx|
      if loc == '^'
        new_tachs[idx - 1] += tachs[idx]
        new_tachs[idx + 1] += tachs[idx]
      else
        new_tachs[idx] += tachs[idx]
      end
    end

    tachs = new_tachs
  end

  tachs.sum
end

hardcoded_data = { }

input_name = ARGV[0]

data =
  if hardcoded_data.has_key?(input_name)
    hardcoded_data[input_name]
  else
    folder = File.dirname(__FILE__)
    path = File.expand_path(File.join(folder, "../#{input_name}"))
    File.read(path)
  end

data = parse(data)

puts part1(*data)
puts part2(*data)
