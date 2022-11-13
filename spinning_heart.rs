fn main() {
  // Clear screen and hide cursor
  print!("\x1b[2J\x1b[?25l");

  let mut t : f64 = 0.0;
  loop {
    let mut zb = [0.0; 100 * 40];
    let mut maxz = 0.0;
    let (c, s) = (t.cos(), t.sin());

    for y in -50..50 {
      let fy = f64::from(y) / 100.0;

      // Add beating effect
      let r = 0.4 + 0.05 * (0.5 + 0.5 * (t * 6.0 + fy * 2.0).sin()).powi(8);

      for x in -50..50 {
        let fx = f64::from(x) / 100.0;

        // Heart formula
        let z = (-fx * fx - (1.2 * fy - fx.abs() * 2.0 / 3.0).powi(2) + r * r).sqrt() / (2.0 - fy);
        if z < 0.0 {
          continue;
        }

        let mut tz = -z;
        while tz < z {
          // Rotate
          let nx = fx * c - tz * s;
          let nz = fx * s + tz * c;

          // Add perspective 
          let p = 1.0 + nz / 2.0;
          // Convert to screen coordinates
          let vx = ((nx * p + 0.5) * 80.0 + 10.0).round();
          let vy = ((-fy * p + 0.5) * 39.0 + 2.0).round();

          let idx = (vx + 100.0 * vy) as usize;
          if zb[idx] <= nz {
            zb[idx] = nz;
            if nz > maxz {
              maxz = nz;
            }
          }

          tz+= z / 6.0;
        }
      }
    }

    let mut d = [0u8; 100 * 40];
    for (i, z) in zb.iter().enumerate() {
      d[i] = if i % 100 == 0 {
          10
        } else {
          let str = " .,-~:;=!*#$@@".as_bytes();
          str[(z / maxz * 13.0).round() as usize]
        };
    };
    print!("\x1b[H{}", std::str::from_utf8(&d).unwrap());

    t+= 0.003;
    std::thread::sleep(std::time::Duration::from_millis(3));
  }
}
