def parse(input)
  input.strip.lines.map do |l|
    dir, amt = l.scan(/(.)(\d+)/)[0]

    amt = Integer(amt)

    [dir, amt]
  end
end

def part1(data)
  arrow = 50
  counter = 0

  data.each do |dir, amt|
    case dir
    when 'L'
      arrow = (arrow - amt) % 100
    when 'R'
      arrow = (arrow + amt) % 100
    else
      raise "unrecognized direction #{dir}"
    end

    counter += 1 if arrow == 0

    #puts "The dial is rotated #{dir}#{amt} to point at #{arrow}"
    #puts "  the counter is now #{counter}" if arrow == 0
  end

  counter
end

def part2(data)
  arrow = 50
  counter = 0

  data.each do |dir, amt|
    #last_counter = counter

    counter += amt / 100
    remaining = amt % 100

    case dir
    when 'L'
      new_arrow = (arrow - amt) % 100
      counter += 1 if arrow != 0 && (new_arrow > arrow || new_arrow == 0)
    when 'R'
      new_arrow = (arrow + amt) % 100
      counter += 1 if new_arrow < arrow
    else
      raise "unrecognized direction #{dir}"
    end

    arrow = new_arrow

    #puts "The dial is rotated #{dir}#{amt} to point at #{arrow}"
    #puts "  the counter is now #{counter}" if last_counter != counter
  end

  counter
end

input = File.read(File.expand_path("../#{ARGV[0]}"))
input = parse(input)

puts part1(input)
puts part2(input)
