require "debug"

def parse(input)
  input.lines.map do |line|
    line.strip.chars.to_a
  end
end

def remove(data)
  removed = 0
  new = []

  data.each_with_index do |line, y|
    new_line = []

    line.each_with_index do |loc, x|
      if loc == '.'
        new_line << '.'
        next
      end

      count = 0

      Range.new([y - 1, 0].max, [y + 1, data.length - 1].min).each do |dy|
        Range.new([x - 1, 0].max, [x + 1, line.length - 1].min).each do |dx|
          next if dy == y && dx == x
          count += 1 if data[dy][dx] == '@'
        end
      end

      if count < 4
        removed += 1
        new_line << '.'
      else
        new_line << '@'
      end
    end

    new << new_line
  end

  [new, removed]
end

def part1(data)
  _, count = remove(data)
  count
end

def part2(data)
  total = 0

  loop do
    data, count = remove(data)
    total += count
    break if count == 0
  end

  total
end

test1 = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@."

hardcoded_data = {
  "test1" => test1,
}

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

puts part1(data)
puts part2(data)
