require "debug"

def parse(input)
  ranges = []
  values = []

  ranges_finished = false

  input.lines.map(&:strip).each do |line|
    ranges_finished = true and next if line.length == 0

    if ranges_finished
      values << Integer(line)
    else
      fst, snd = line.split('-')
      ranges << Range.new(Integer(fst), Integer(snd))
    end
  end

  [ranges, values]
end

def combine_range(fst, snd)
  Range.new([fst.begin, snd.begin].min, [fst.end, snd.end].max)
end

def reduce_ranges(ranges)
  new_ranges = []

  ranges.each do |r|
    loop do
      n = new_ranges.index { |x| x.overlap?(r) }

      if n
        r = combine_range(new_ranges.delete_at(n), r)
      else
        new_ranges << r
        break
      end
    end
  end

  new_ranges
end

def part1(ranges, values)
  fresh = 0

  values.each do |v|
    fresh += 1 if ranges.any? { |r| r.include?(v) }
  end

  fresh
end

def part2(ranges)
  ranges.map(&:count).sum
end

input = File.read(File.expand_path("../#{ARGV[0]}"))
ranges, values = parse(input)
ranges = reduce_ranges(ranges)

puts part1(ranges, values)
puts part2(ranges)
