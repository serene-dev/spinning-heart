
import java.lang.Math;
import java.util.concurrent.TimeUnit;

public class SpinningHeart {
  public static void main(String[] main) throws InterruptedException {
    // Clear screen hide cursor
    System.out.print("\033[2J\033[?25l");

    float t = 0;
    while (true) {
      double zb[] = new double[100 * 40];
      double maxz = 0, c = Math.cos(t), s = Math.sin(t);
      for (double y = -0.5; y <= 0.5; y+= 0.01) {
        // Add beating effect
        double r = 0.4 + 0.05 * Math.pow(0.5 + 0.5 * Math.sin(t * 6 + y * 2), 8);
        for (double x = -0.5; x <= 0.5; x+= 0.01) {
          // heart formula
          double z = -x * x - Math.pow(1.2 * y - Math.abs(x) * 2 / 3, 2) + r * r;
          if (z < 0)
            continue;
          z = Math.sqrt(z) / (2 - y);
          for (double tz = -z; tz <= z; tz+= z / 6) {
            // Rotate
            double nx = x * c - tz * s;
            double nz = x * s + tz * c;

            // Add perspective
            double p = 1 + nz / 2;
            int vx = (int)Math.round((nx * p + 0.5) * 80 + 10);
            int vy = (int)Math.round((-y * p + 0.5) * 39 + 2);
            int idx = vx + vy * 100;
            if (zb[idx] <= nz) {
              zb[idx] = nz;
              if (maxz <= nz)
                maxz = nz;
            }
          }
        }
      }

      System.out.print("\033[H");
      for (int i = 0; i < 100 * 40; i++)
        System.out.print(i % 100 != 0 ? " .,-~:;=!*#$@@".charAt((int)Math.round(zb[i] / maxz * 13)) : 10);

      t+= 0.003f;
      TimeUnit.MILLISECONDS.sleep(3);
    }
  }
};
