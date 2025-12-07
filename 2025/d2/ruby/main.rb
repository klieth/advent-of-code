require "time"

def report_timed(&block)
  mark = Time.now
  res = yield
  elapsed = Time.now - mark

  puts "#{res}: in #{elapsed} seconds"
end

def parse(input)
  input.strip.split(',').map do |raw|
    a, b = raw.split('-')

    Integer(a)..Integer(b)
  end
end

def part1(data)
  count = 0

  data.each do |range|
    range.each do |value|
      length = (Math.log10(value) + 1).floor
      next if length % 2 == 1 # definitionally can't be odd

      divisor = 10 ** (length / 2)

      count += value if (value / divisor) == (value % divisor)
    end
  end

  count
end

def part2(data)
  count = 0

  data.each do |range|
    range.each do |value|
      length = (Math.log10(value) + 1).floor

      (1..(length / 2)).each do |l|
        divisor = 10 ** l
        next if length % l != 0

        s = Set.new
        v = value

        while v > 0
          s << v % divisor
          v = v / divisor
        end

        if s.length == 1
          count += value
          break
        end
      end
    end
  end

  count
end

data = parse(File.read(File.expand_path("../#{ARGV[0]}")))

report_timed { part1(data) }
report_timed { part2(data) }
