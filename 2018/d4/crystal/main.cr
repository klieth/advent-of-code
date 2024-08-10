
class Guard
  class Action
    CHANGE = /Guard #(\d+) begins shift/
    SLEEP = /falls asleep/
    WAKE = /wakes up/
  end

  property current : Int32? = nil
  getter sleeps : Array(Range(Int32, Int32)) = [] of Range(Int32, Int32)

  def sleep(at : Int32) : Nil
    if @current.nil?
      @current = at
    else
      raise "attempt to sleep when already sleeping"
    end
  end

  def wake(at : Int32) : Nil
    start = @current || raise "attempt to wake when not asleep"
    @sleeps << (start...at)
    @current = nil
  end

  def strategy1 : {Int32, Int32}
    time_slept = 0
    minutes = Array.new(60, 0)

    @sleeps.each do |sleep|
      time_slept += sleep.size

      sleep.each do |minute|
        minutes[minute] += 1
      end
    end

    max_minute = 0
    max_count = 0

    minutes.each_with_index do |m, i|
      if m > max_count
        max_minute = i
        max_count = m
      end
    end

    {time_slept, max_minute}
  end

  def strategy2 : {Int32, Int32}
    minutes = Array.new(60, 0)

    @sleeps.each do |sleep|
      sleep.each do |minute|
        minutes[minute] += 1
      end
    end

    max_minute = 0
    max_count = 0

    minutes.each_with_index do |m, i|
      if m > max_count
        max_minute = i
        max_count = m
      end
    end

    {max_count, max_minute}
  end
end

def parse(raw : String) : Hash(Int32, Guard)
  data = raw.each_line.map do |line|
    md = /\[([^]]+)\] (.*)/.match(line)
    if md
      time = Time.parse(md[1], "%Y-%m-%d %H:%M", Time::Location::UTC)
      {time, md[2]}
    else
      raise "failed to match"
    end
  end.to_a.sort_by do |line|
    line[0]
  end

  _, first_action = data.shift

  guards = {} of Int32 => Guard

  first_change = Guard::Action::CHANGE.match(first_action) || raise "first action must be CHANGE but got: #{first_action}"

  current_guard : Int32 = first_change[1].to_i
  guards[current_guard] = Guard.new

  data.each do |ts, action|
    case action
    when Guard::Action::CHANGE
      current_guard = $~[1].to_i
      guards[current_guard] ||= Guard.new
    when Guard::Action::SLEEP
      guards[current_guard].sleep(ts.minute)
    when Guard::Action::WAKE
      guards[current_guard].wake(ts.minute)
    else
      raise "unknown action: #{action}"
    end
  end

  guards
end

def calc(guards : Hash(Int32, Guard), &strategy : Guard -> {Int32, Int32}) : Int32
  guard_num, _, sleepiest_minute = guards.map do |num, guard|
    {num, *yield(guard)}
  end.max_by do |res|
    res[1]
  end

  guard_num * sleepiest_minute
end

def main : Nil
  raw = File.open("../input") do |f|
    f.gets_to_end
  end

  data = parse(raw)

  puts calc(data, &.strategy1)
  puts calc(data, &.strategy2)
end

main
