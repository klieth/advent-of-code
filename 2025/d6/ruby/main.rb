require "debug"

def parse1(input)
  parsed = input.lines.map do |line|
    line.strip.split(/\s+/)
  end

  ops = parsed.pop
  parsed.each do |row|
    row.map! { Integer(it) }
  end

  [parsed, ops]
end

def part1(data)
  parsed, ops = parse1(data)

  ops.map.with_index do |op, index|
    parsed
      .map { it[index] }
      .reduce { |a, b| a.send(op, b) }
  end.sum
end

def part2(data)
  lines = data.lines.map { it.chars.to_a.tap { it.pop } }.to_a
  raise "lines are the wrong length" unless lines.all? { it.length == lines[0].length }
  ops = lines.pop

  sum = 0
  nums = []

  while lines[0].length > 0
    num = lines.map { it.pop }.join.strip
    ops.pop and next if num == ""

    nums << Integer(num)

    op = ops.pop
    if op != ' '
      sum += nums.reduce { |a, b| a.send(op, b) }
      nums = []
    end
  end

  sum
end

# 123 328  51 64 
#  45 64  387 23 
#   6 98  215 314
# *   +   *   +

test1 = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +   "

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

puts part1(data)
puts part2(data)
