# Prompt #5: Camera at (0,0,0) looks toward +Z. Wall #1 is a 4 m × 4 m square 10 m away. Wall #2 is a 2 m × 2 m square 5 m away, centered exactly in front of Wall #1. What percent of Wall #1’s area is visible?

## OpenAI o3
From the camera at the origin every point is seen along a straight line (ray) that passes through the origin.  
Put the two walls in the (x , y , z)‐coordinate system:

Wall #1 (the back wall)  
• Plane z = 10 m  
• Square: –2 m ≤ x ≤ 2 m, –2 m ≤ y ≤ 2 m  (size 4 m × 4 m)

Wall #2 (the front wall)  
• Plane z = 5 m  
• Square: –1 m ≤ x ≤ 1 m, –1 m ≤ y ≤ 1 m  (size 2 m × 2 m)

Perspective projection of Wall #2 onto the plane z = 10:
For any point (x , y , 5) on Wall #2 the ray through the origin is  
t·(x , y , 5) , t ≥ 1.  
To reach z = 10 we need 5 t = 10  ⇒  t = 2, giving the point  
(2x , 2y , 10).

Hence every coordinate is doubled; the 2 m × 2 m square at z = 5 is mapped to a 4 m × 4 m square at z = 10 whose extents are –2 m ≤ x ≤ 2 m, –2 m ≤ y ≤ 2 m—the same extents as Wall #1.

Therefore the projected outline of the nearer 2 m × 2 m wall coincides exactly with the outline of the farther 4 m × 4 m wall. Every ray that reaches any interior point of Wall #1 intersects Wall #2 first, so Wall #2 completely occludes Wall #1 (except along the boundary, which has zero area).

Visible area of Wall #1 = 0  
Total area of Wall #1 = 4 m × 4 m = 16 m²

Percentage visible = 0 / 16 × 100 % = 0 %.

Answer: 0 % of Wall #1 is visible.

## Claude 4 Sonnet
I need to find what percentage of Wall #1 is visible when Wall #2 is blocking part of it.

Given information:
- Camera at origin (0,0,0) looking toward +Z
- Wall #1: 4m × 4m square at z = 10m
- Wall #2: 2m × 2m square at z = 5m
- Both walls are centered (aligned on the z-axis)

Since both walls are centered and the camera is at the origin looking along the +Z axis, this is a similar triangles problem.

For Wall #2 (2m × 2m at z = 5m):
- The edges are at ±1m in both x and y directions
- From the camera, the angular extent is: tan(θ) = 1/5

For Wall #1 (4m × 4m at z = 10m):
- The edges are at ±2m in both x and y directions
- From the camera, the angular extent is: tan(θ) = 2/10 = 1/5

Since both walls subtend the same angle from the camera (1/5 for the half-angle), Wall #2 exactly covers the central 2m × 2m portion of Wall #1 when viewed from the camera.

The shadow of Wall #2 on Wall #1:
- At z = 10m, the shadow has the same angular size as Wall #2
- Shadow dimensions: 2m × 2m (centered on Wall #1)

Visible area calculation:
- Total area of Wall #1: 4m × 4m = 16 m²
- Area blocked by Wall #2's shadow: 2m × 2m = 4 m²
- Visible area of Wall #1: 16 - 4 = 12 m²

Percentage visible: (12/16) × 100% = 75%

Therefore, 75% of Wall #1's area is visible.

## Gemini Pro
Let's analyze the situation using similar triangles.

1.  **Camera Position:** (0,0,0)
2.  **Looking Direction:** +Z

3.  **Wall #2 (Occluding Wall):**
    *   Distance from camera (z_2): 5 m
    *   Dimensions: 2 m × 2 m
    *   Since it's centered, its x-coordinates range from -1 m to +1 m, and y-coordinates range from -1 m to +1 m at z = 5 m.

4.  **Wall #1 (Background Wall):**
    *   Distance from camera (z_1): 10 m
    *   Dimensions: 4 m × 4 m
    *   Since it's centered, its x-coordinates range from -2 m to +2 m, and y-coordinates range from -2 m to +2 m at z = 10 m.
    *   Total area of Wall #1 (A_1) = 4 m × 4 m = 16 m².

5.  **Projection of Wall #2 onto Wall #1:**
    We need to find out how large the "shadow" or projection of Wall #2 is on the plane of Wall #1.
    Consider one dimension, say the width (x-dimension).
    The half-width of Wall #2 is w_2_half = 2 m / 2 = 1 m.
    The distance to Wall #2 is z_2 = 5 m.
    The distance to Wall #1 is z_1 = 10 m.

    Let W_shadow_half be the half-width of the shadow cast by Wall #2 on the plane of Wall #1.
    Using similar triangles (formed by the camera, the edge of Wall #2, and its projection on Wall #1):
    w_2_half / z_2 = W_shadow_half / z_1
    1 m / 5 m = W_shadow_half / 10 m
    W_shadow_half = (1 m / 5 m) * 10 m = 2 m.

    So, the full width of the shadow cast by Wall #2 on the plane of Wall #1 is 2 * W_shadow_half = 2 * 2 m = 4 m.

    Similarly, for the height (y-dimension), the half-height of Wall #2 is h_2_half = 1 m.
    The shadow's half-height H_shadow_half on Wall #1 will also be:
    h_2_half / z_2 = H_shadow_half / z_1
    1 m / 5 m = H_shadow_half / 10 m
    H_shadow_half = 2 m.

    So, the full height of the shadow cast by Wall #2 on the plane of Wall #1 is 2 * H_shadow_half = 2 * 2 m = 4 m.

6.  **Area of Occlusion:**
    The shadow cast by Wall #2 on the plane of Wall #1 is 4 m wide and 4 m high.
    Area of occlusion (A_occluded) = 4 m × 4 m = 16 m².

7.  **Visible Area of Wall #1:**
    The total area of Wall #1 is A_1 = 16 m².
    The area occluded by Wall #2 is A_occluded = 16 m².
    Visible area (A_visible) = A_1 - A_occluded
    A_visible = 16 m² - 16 m² = 0 m².

8.  **Percent of Wall #1's Area Visible:**
    Percent visible = (A_visible / A_1) * 100%
    Percent visible = (0 m² / 16 m²) * 100% = 0%.

Wall #1 is completely occluded by Wall #2.

Final Answer: 0%

## DeepSeek Reasoner
The camera is positioned at (0,0,0) and looks toward the positive Z-axis. Wall #1 is a 4 m × 4 m square centered at (0,0,10), so its corners are at (±2, ±2, 10). Wall #2 is a 2 m × 2 m square centered at (0,0,5), so its corners are at (±1, ±1, 5).

The angular size of both walls from the camera is identical. The half-width of Wall #1 at 10 m is 2 m, giving \(\tan(\theta) = 2/10 = 0.2\), so \(\theta \approx 11.31^\circ\). The half-width of Wall #2 at 5 m is 1 m, giving \(\tan(\phi) = 1/5 = 0.2\), so \(\phi \approx 11.31^\circ\). Thus, both walls subtend the same angle from the camera.

For any point on Wall #1, the line of sight from the camera to that point passes through the plane at Z=5. Specifically, a point \((x_1, y_1, 10)\) on Wall #1 intersects the plane Z=5 at \((0.5x_1, 0.5y_1, 5)\). Since Wall #1 has \(|x_1| \leq 2\) and \(|y_1| \leq 2\), the intersection point satisfies \(|0.5x_1| \leq 1\) and \(|0.5y_1| \leq 1\), meaning it lies within the area of Wall #2. Therefore, Wall #2 blocks the line of sight to every point on Wall #1.

As a result, no part of Wall #1 is visible from the camera. The area of Wall #1 is 16 m², and the visible area is 0 m², so the percentage of visible area is 0%.

\boxed{0}

## Grok 3
To determine the percentage of Wall #1’s area that is visible, we need to calculate how much of Wall #1 is obscured by Wall #2 and subtract that from the total area of Wall #1. Let's solve this step by step.

---

### Step 1: Define the setup and coordinates
- The camera is at the origin, \((0, 0, 0)\), looking toward the positive Z-axis.
- Wall #1 is a 4 m × 4 m square located 10 m away along the Z-axis. Since it's centered on the Z-axis (implied by the problem's symmetry and Wall #2 being "centered exactly in front of Wall #1"), its coordinates in the XY-plane at \(z = 10\) m span from \(x = -2\) m to \(x = 2\) m and \(y = -2\) m to \(y = 2\) m.
- Wall #2 is a 2 m × 2 m square located 5 m away along the Z-axis. Since it's centered in front of Wall #1, its coordinates in the XY-plane at \(z = 5\) m span from \(x = -1\) m to \(x = 1\) m and \(y = -1\) m to \(y = 1\) m.

The total area of Wall #1 is:
\[
\text{Area of Wall #1} = 4 \, \text{m} \times 4 \, \text{m} = 16 \, \text{m}^2.
\]

Our goal is to find how much of Wall #1 is visible from the camera's perspective, i.e., not blocked by Wall #2.

---

### Step 2: Understand visibility from the camera's perspective
Since the camera is at the origin and looking along the Z-axis, visibility is determined by the projection of Wall #2 onto Wall #1. If a ray from the camera passes through Wall #2 before reaching Wall #1, that part of Wall #1 is obscured. Because both walls are perpendicular to the Z-axis and centered on it, we can use similar triangles to determine the shadow (or projection) of Wall #2 onto Wall #1.

- Wall #2 is at \(z = 5\) m, and Wall #1 is at \(z = 10\) m.
- The camera is at \(z = 0\) m.
- The distance from the camera to Wall #2 is 5 m, and to Wall #1 is 10 m, so the distance ratio is \(10/5 = 2\).

Since the walls are parallel to the XY-plane and the camera is at the origin, the projection of Wall #2 onto Wall #1 scales linearly with the distance ratio. Wall #2's dimensions (2 m × 2 m) at \(z = 5\) m will project to a larger area on Wall #1 at \(z = 10\) m.

The scaling factor for the projection is the ratio of the distances:
\[
\text{Scaling factor} = \frac{\text{Distance to Wall #1}}{\text{Distance to Wall #2}} = \frac{10}{5} = 2.
\]

Thus, the projection of Wall #2 onto Wall #1 will be:
- Width: \(2 \, \text{m} \times 2 = 4 \, \text{m}\),
- Height: \(2 \, \text{m} \times 2 = 4 \, \text{m}\).

So, the projected shadow of Wall #2 on Wall #1 is a 4 m × 4 m square.

---

### Step 3: Compare the projection to Wall #1's dimensions
Wall #1 is exactly 4 m × 4 m, and the projection of Wall #2 onto Wall #1 is also 4 m × 4 m. Since both walls are centered on the Z-axis, the projection of Wall #2 completely covers Wall #1.

- Wall #1 extends from \(x = -2\) to \(2\) and \(y = -2\) to \(2\) at \(z = 10\).
- Wall #2 extends from \(x = -1\) to \(1\) and \(y = -1\) to \(1\) at \(z = 5\). Its projection onto \(z = 10\) scales by a factor of 2, so it extends from \(x = -2\) to \(2\) and \(y = -2\) to \(2\), exactly matching Wall #1's dimensions.

Thus, the entire area of Wall #1 is obscured by Wall #2 from the camera's perspective.

---

### Step 4: Calculate the visible area
Since the projection of Wall #2 completely covers Wall #1:
- Visible area of Wall #1 = Total area of Wall #1 − Area obscured by Wall #2's projection.
- Visible area = \(16 \, \text{m}^2 - 16 \, \text{m}^2 = 0 \, \text{m}^2\).

Therefore, the percentage of Wall #1's area that is visible is:
\[
\text{Percentage visible} = \left( \frac{\text{Visible area}}{\text{Total area of Wall #1}} \right) \times 100 = \left( \frac{0}{16} \right) \times 100 = 0\%.
\]

---

### Step 5: Double-check the reasoning
The result of 0% visible area seems counterintuitive at first because one might expect some part of Wall #1 to be visible around the edges. However, the projection calculation using similar triangles confirms that Wall #2's shadow scales exactly to cover Wall #1:
- A point on the edge of Wall #2 at \(x = 1, y = 1, z = 5\) projects along the ray from the camera at \((0,0,0)\) to this point. The ray continues to \(z = 10\), where \(x = 2, y = 2\) (since \(10/5 = 2\), scaling the coordinates by 2).
- This matches the edge of Wall #1 (\(x = 2, y = 2\)).

Thus, the projection of Wall #2 exactly covers Wall #1, leaving no visible area.

---

### Final Answer
**0%** of Wall #1’s area is visible.

