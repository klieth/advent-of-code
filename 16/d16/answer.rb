input = ARGV.first || '01111010110010011'

[272, 35651584].each { |disk|
  # The disk pattern is:
  # input, joiner, input reversed and negated, joiner, repeat
  a = input.each_char.map { |c| c == ?1 }.freeze
  a_rev = a.reverse.map { |x| !x }.freeze

  joiners = []
  until joiners.size * (a.size + 1) >= disk
    joiners += [false] + joiners.reverse.map { |x| !x }
  end

  # chunk_size: the largest power of 2 that divides disk.
  # e.g.   272 is 100010000
  #        271 is 100001111
  #       ~271 is  11110000
  # 272 & ~271 is     10000
  chunk_size = disk & ~(disk - 1)
  sum_size = disk / chunk_size

  buf = []

  # each character in the final checksum
  # corresponds to `chunk_size` consecutive characters on disk.
  puts sum_size.times.map { |c|
    # Anything left in the buffer from last time?
    take_from_buffer = [buf.size, chunk_size].min
    remaining = chunk_size - take_from_buffer
    ones = buf.shift(take_from_buffer).count(true)

    # How many full AJAJ groups will we have?
    full_ajajs, remaining = remaining.divmod((a.size + 1) * 2)
    # Count all the ones in the joiners.
    ones += joiners.shift(full_ajajs * 2).count(true)
    # The number of ones in a + a_rev... is obviously a.size.
    ones += a.size * full_ajajs

    if remaining > 0
      buf.concat(a)
      buf << joiners.shift
      buf.concat(a_rev)
      buf << joiners.shift
      ones += buf.shift(remaining).count(true)
    end

    ones % 2 == 0 ? ?1 : ?0
  }.join
}
