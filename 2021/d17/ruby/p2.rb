ltx = 67
gtx = 34

lty = -186
gty = -215

count = 0

(8..67).each do |start_vx|
  (-215..214).each do |start_vy|
    vx = start_vx.dup
    vy = start_vy.dup
    x = 0
    y = 0

    while y >= gty && x <= ltx
      x += vx
      y += vy

      vx -= 1 if vx > 0
      vy -= 1

      if x <= ltx && x >= gtx && y <= lty && y >= gty
        count += 1
        break
      end
    end
  end
end

puts "count: #{count}"
