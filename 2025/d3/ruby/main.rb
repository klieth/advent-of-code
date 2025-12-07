require "debug"

def parse(input)
  input.strip.lines.map(&:strip).to_a
end

def part1(data)
  data.map do |line|
    first, idx = line[0..-2].chars.each_with_index.max_by { |x, _| x }
    second = line[idx+1..-1].chars.max
    Integer(first + second)
  end.sum
end

def part2(data)
  data.map do |line|
    indexed = line.chars.each_with_index.to_a
    start = 0
    final = []

    (1..12).to_a.reverse.each do |place|
      chr, idx = indexed[start..-place].max_by { |x, _| x }
      final << chr
      start = idx + 1
    end

    Integer(final.join)
  end.sum
end

test1 = "987654321111111
811111111111119
234234234234278
818181911112111"

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
