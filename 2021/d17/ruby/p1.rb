vx = 8
vy = Integer(ARGV[0])

ltx = 67
gtx = 34

lty = -186
gty = -215

x = 0
y = 0

max = 0
hit_target = false

while y >= gty
  x += vx
  y += vy

  vx -= 1 if vx > 0
  vy -= 1

  max = y if vy == 0
  hit_target = true if hit_target || (x <= ltx && x >= gtx && y <= lty && y >= gty)

  puts "loc: #{x},#{y} -- v: #{vx},#{vy}"
end

puts "max: #{max}, hit: #{hit_target}"
