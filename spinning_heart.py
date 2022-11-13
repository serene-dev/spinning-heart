
import time, math

def main():
  # Clear screen hide cursor
  print("\x1b[2J\x1b[?25l", end='')
  zb = [0.0] * 100 * 40

  t = 0.0
  while True:
    maxz, c, s = 0, math.cos(t), math.sin(t)
    y = -0.5
    while y <= 0.5:
      # Add beating effect
      r = 0.4 + 0.05 * math.pow(0.5 + 0.5 * math.sin(t * 6 + y * 2), 8)
      x = -0.5
      while x <= 0.5:
        # heart formula
        z = -x * x - math.pow(1.2 * y - abs(x) * 2 / 3, 2) + r * r
        if z >= 0:
          z = math.sqrt(z) / (2 - y)
          tz = -z
          while tz <= z:
            # Rotate
            nx = x * c - tz * s
            nz = x * s + tz * c

            # Add perspective
            p = 1 + nz / 2
            vx = round((nx * p + 0.5) * 80 + 10)
            vy = round((-y * p + 0.5) * 39 + 2)
            idx = vx + vy * 100
            if zb[idx] <= nz:
              zb[idx] = nz
              if maxz < nz:
                maxz = nz
            tz+= z / 6
        x+= 0.01
      y+= 0.01

    print("\x1b[H", end='')
    for i in range(100 * 40):
      print(i % 100 and " .,-~:;=!*#$@@"[round(zb[i] / maxz * 13)] or "\n", end='')
      zb[i] = 0

    t+= 0.003
    time.sleep(0.003)

if __name__ == "__main__":
  main()
