# Prompt #2: A point is at (2, 1, −3) in camera space. The camera is at (0, 0, 0) looking down −Z with a 90° FOV. What pixel row/column (in a 1920 × 1080 sensor) does the point project to?

## OpenAI o3
Camera-space point :  (x , y , z) = ( 2 , 1 , –3 )  

1. Perspective-divide to get the point on the image plane ( z = –1 is used )  
   xʹ = x / –z = 2 / 3 = 0.666…  
   yʹ = y / –z = 1 / 3 = 0.333…

2. Field of view  
   The given FOV (90°) is taken to be the vertical field–of–view.  
   tan(FOV/2) = tan 45° = 1,   so on the image plane  
        y ranges from –1 (bottom) to +1 (top).

3. Horizontal extent (aspect ratio 1920:1080 = 16:9)  
   halfWidth = tan(FOV/2) · aspect = 1 · 16/9 = 16/9 ≈ 1.7778,  
   so x ranges from –16/9 to +16/9.

4. Convert to normalised image coordinates (0 at left / top, 1 at right / bottom)

   nx = (xʹ + halfWidth) / (2·halfWidth)  
      = ( 2/3 + 16/9 ) / ( 32/9 )  
      = (22/9)/(32/9) = 22/32 = 11/16 = 0.6875  

   ny = (top − yʹ) / (2·tan(FOV/2))  
      = ( 1 − 1/3 ) / 2 = (2/3)/2 = 1/3 ≈ 0.3333

5. Map to pixels (0-based indices)

   column = nx · 1920 ≈ 0.6875 · 1920 = 1320  
   row    = ny · 1080 ≈ 0.3333 · 1080 = 360

So the point is seen at pixel column 1320, row 360 on a 1920 × 1080 sensor (origin at the top-left corner).

## Claude 4 Sonnet
I need to project a 3D point from camera space to pixel coordinates.

Given information:
- Point in camera space: P = (2, 1, -3)
- Camera at origin (0, 0, 0) looking down -Z
- FOV = 90°
- Image resolution: 1920 × 1080

Step 1: Check if the point is in front of the camera.
Since the camera looks down -Z, and the point has z = -3, it's in front of the camera (good).

Step 2: Calculate the projection to normalized device coordinates.
For a perspective projection with 90° FOV:
- FOV = 90°, so FOV/2 = 45°
- tan(45°) = 1

The projection formulas are:
- x' = x / (-z) = 2 / 3 = 2/3
- y' = y / (-z) = 1 / 3 = 1/3

Step 3: Convert to normalized device coordinates (NDC).
With a 90° FOV and tan(45°) = 1, the projected coordinates are already in NDC range.
- x_ndc = 2/3
- y_ndc = 1/3

Step 4: Convert NDC to pixel coordinates.
The conversion formulas are:
- column = (x_ndc + 1) × width / 2 = (2/3 + 1) × 1920 / 2 = (5/3) × 960 = 1600
- row = (1 - y_ndc) × height / 2 = (1 - 1/3) × 1080 / 2 = (2/3) × 540 = 360

Therefore, the point projects to:
- Column: 1600
- Row: 360

## Gemini Pro
Okay, let's break this down step-by-step.

**1. Understand the Setup:**
*   Camera is at origin (0,0,0).
*   Looking down the -Z axis. This is standard for camera space. Points in front of the camera will have negative Z values.
*   Point P_cam = (2, 1, -3). (x_c = 2, y_c = 1, z_c = -3)
*   FOV = 90°. By convention, if not specified as horizontal or vertical, FOV usually refers to the **vertical FOV (fov_y)**.
*   Sensor resolution: 1920 pixels wide (W), 1080 pixels high (H).

**2. Aspect Ratio:**
The aspect ratio (AR) of the sensor is:
AR = Width / Height = 1920 / 1080 = 16 / 9

**3. Projection to Normalized Device Coordinates (NDC):**
In perspective projection, the projected coordinates (x_p, y_p) on a conceptual projection plane at distance `d` (focal length) from the camera are given by similar triangles:
x_p / d = x_c / (-z_c)  => x_p = d * x_c / (-z_c)
y_p / d = y_c / (-z_c)  => y_p = d * y_c / (-z_c)
(We use -z_c because z_c is negative, and we want a positive distance in the denominator for the ratio).

To get to Normalized Device Coordinates (NDC), which typically range from -1 to 1, we relate `d` to the FOV.
For vertical FOV (fov_y):
`tan(fov_y / 2) = (height_of_projection_plane_at_d / 2) / d`
If the NDC y-coordinate goes from -1 to 1, then `y_ndc = y_p / (height_of_projection_plane_at_d / 2)`.
Combining these, `y_ndc = (d * y_c / (-z_c)) / (d * tan(fov_y / 2)) = y_c / (-z_c * tan(fov_y / 2))`

Similarly for x_ndc, we need the horizontal FOV (fov_x). The relationship is:
`tan(fov_x / 2) = AR * tan(fov_y / 2)`
So, `x_ndc = x_c / (-z_c * tan(fov_x / 2)) = x_c / (-z_c * AR * tan(fov_y / 2))`

Given:
*   fov_y = 90°
*   tan(fov_y / 2) = tan(45°) = 1

Now calculate NDC coordinates:
*   `y_ndc = y_c / (-z_c * tan(fov_y/2))`
    `y_ndc = 1 / (-(-3) * 1)`
    `y_ndc = 1 / (3 * 1) = 1/3`

*   `x_ndc = x_c / (-z_c * AR * tan(fov_y/2))`
    `x_ndc = 2 / (-(-3) * (16/9) * 1)`
    `x_ndc = 2 / (3 * 16/9)`
    `x_ndc = 2 / (16/3)`
    `x_ndc = 2 * 3 / 16 = 6 / 16 = 3/8`

So, P_ndc = (3/8, 1/3).

**4. Convert NDC to Pixel Coordinates (Raster Space):**
NDC coordinates typically range from -1 to 1 for both x and y.
*   x_ndc = -1 maps to pixel column 0.
*   x_ndc = +1 maps to pixel column W-1 (or W for mapping range).
*   y_ndc = -1 (bottom) maps to pixel row H-1 (or H). (Assuming pixel origin (0,0) is top-left)
*   y_ndc = +1 (top) maps to pixel row 0.

The transformation formulas are:
*   `pixel_col = (x_ndc + 1) / 2 * W`
*   `pixel_row = (1 - y_ndc) / 2 * H` (The `1 - y_ndc` handles the flip in the Y-axis direction between NDC (Y up) and typical screen coordinates (Y down)).

Let W = 1920, H = 1080.
*   `pixel_col = (3/8 + 1) / 2 * 1920`
    `pixel_col = (11/8) / 2 * 1920`
    `pixel_col = (11/16) * 1920`
    `pixel_col = 11 * (1920 / 16)`
    `pixel_col = 11 * 120 = 1320`

*   `pixel_row = (1 - 1/3) / 2 * 1080`
    `pixel_row = (2/3) / 2 * 1080`
    `pixel_row = (1/3) * 1080`
    `pixel_row = 360`

**Result:**
The point (2, 1, -3) in camera space projects to:
*   **Pixel Column: 1320**
*   **Pixel Row: 360**

(Assuming pixel coordinates are 0-indexed, so column 1320 is the 1321st column, and row 360 is the 361st row).

## DeepSeek Reasoner
The point is at (2, 1, -3) in camera space, with the camera at (0, 0, 0) looking down the negative Z-axis. The vertical field of view (FOV) is 90°, and the sensor resolution is 1920 × 1080 (width × height).

### Projection Calculation:
1. **Normalized Device Coordinates (NDC):**  
   - The projection formulas, considering the vertical FOV of 90° and aspect ratio (width/height = 1920/1080 = 16/9), are:  
     \[
     x_{\text{ndc}} = \frac{x}{-z \cdot \text{aspect} \cdot \tan(\theta/2)}, \quad y_{\text{ndc}} = \frac{y}{-z \cdot \tan(\theta/2)}
     \]  
     where \(\theta = 90^\circ\), so \(\tan(\theta/2) = \tan(45^\circ) = 1\).  
   - Plugging in the point \((x, y, z) = (2, 1, -3)\):  
     \[
     x_{\text{ndc}} = \frac{2}{3 \cdot (16/9) \cdot 1} = \frac{2}{3 \cdot 16/9} = \frac{2 \cdot 9}{3 \cdot 16} = \frac{18}{48} = \frac{3}{8} = 0.375,
     \]  
     \[
     y_{\text{ndc}} = \frac{1}{3 \cdot 1} = \frac{1}{3} \approx 0.3333.
     \]

2. **Pixel Coordinates:**  
   - The NDC range \([-1, 1]\) is mapped to pixel coordinates:  
     \[
     \text{column} = \frac{x_{\text{ndc}} + 1}{2} \times (\text{width} - 1), \quad \text{row} = \frac{1 - y_{\text{ndc}}}{2} \times (\text{height} - 1).
     \]  
   - Using width = 1920, height = 1080:  
     \[
     \text{column} = \frac{0.375 + 1}{2} \times 1919 = \frac{1.375}{2} \times 1919 = 0.6875 \times 1919 = 1319.3125,
     \]  
     \[
     \text{row} = \frac{1 - 0.3333}{2} \times 1079 \approx \frac{0.6667}{2} \times 1079 = 0.33335 \times 1079 \approx 359.6667.
     \]

3. **Rounding to Nearest Integer Pixel:**  
   - Column: \(1319.3125\) rounds to **1319** (since \(0.3125 < 0.5\)).  
   - Row: \(359.6667\) rounds to **360** (since \(0.6667 \geq 0.5\)).

### Verification:
- The sensor dimensions are 1920 columns (0 to 1919) and 1080 rows (0 to 1079).  
- Column 1319 is within [0, 1919].  
- Row 360 is within [0, 1079].  
- The point \((2, 1, -3)\) has positive \(x\) and \(y\) in camera space, so it should project to the right of center (column > 960) and above center (row < 540). The result (column 1319, row 360) aligns with this.

### Final Answer:
The point projects to pixel **row 360, column 1319**.  
Thus, the answer is:  
\[
\boxed{(360,\ 1319)}
\]

## Grok 3
To determine the pixel row and column where a 3D point in camera space projects onto a 2D sensor, we need to perform a perspective projection using the camera's field of view (FOV) and the sensor's resolution. Let's break this down step by step.

---

### Step 1: Understand the Camera Setup
- **Camera Position**: The camera is at the origin \((0, 0, 0)\) in camera space.
- **Camera Orientation**: The camera looks down the negative Z-axis (\(-Z\)). This means points with negative Z coordinates are in front of the camera.
- **Field of View (FOV)**: The FOV is 90° (assumed to be the vertical FOV, as is common in many contexts, unless specified otherwise).
- **Sensor Resolution**: The sensor is \(1920 \times 1080\) pixels (width × height).
- **Point in Camera Space**: The point is at \((2, 1, -3)\), meaning it is in front of the camera since \(Z = -3 < 0\).

In camera space, the projection plane is typically at \(Z = -1\) (a common convention for normalized device coordinates or projection matrices). Points are projected onto this plane using perspective projection.

---

### Step 2: Perspective Projection
In perspective projection, a 3D point \((X, Y, Z)\) in camera space is projected to a 2D point \((X', Y')\) on the projection plane using the following formulas:

\[
X' = \frac{X}{Z} \cdot f, \quad Y' = \frac{Y}{Z} \cdot f
\]

where \(f\) is the focal length (or a scaling factor related to the FOV). Since the projection plane is often normalized to \(Z = -1\), and the FOV is given, we can relate the focal length to the FOV.

#### Field of View and Focal Length
For a vertical FOV of 90°, the relationship between the FOV and the focal length \(f\) (assuming the projection plane is at \(Z = -1\)) is:

\[
\text{FOV} = 2 \cdot \arctan\left(\frac{h}{2f}\right)
\]

where \(h\) is the height of the sensor in normalized units. For a 90° FOV, the height of the view frustum at \(Z = -1\) spans from \(-1\) to \(1\) in Y (since \(\tan(45°) = 1\)), so:

\[
f = 1
\]

Thus, the projection simplifies to:

\[
X' = \frac{X}{Z}, \quad Y' = \frac{Y}{Z}
\]

(If the FOV were different, we would adjust \(f\) accordingly, but for 90°, it works out to \(f = 1\).)

Now, substitute the point \((X, Y, Z) = (2, 1, -3)\):

\[
X' = \frac{2}{-3} = -\frac{2}{3} \approx -0.6667
\]
\[
Y' = \frac{1}{-3} = -\frac{1}{3} \approx -0.3333
\]

So, the normalized projected coordinates are \((-0.6667, -0.3333)\). These coordinates are in normalized device coordinates (NDC), where \(X'\) and \(Y'\) range from \(-1\) to \(1\) for points within the view frustum (for a 90° FOV, the edges of the sensor correspond to \(-1\) and \(1\)).

---

### Step 3: Map Normalized Coordinates to Sensor Pixels
The sensor has a resolution of \(1920 \times 1080\) pixels. In NDC, the range \(-1\) to \(1\) corresponds to the full width and height of the sensor. However, we must account for the aspect ratio of the sensor to ensure the mapping preserves the field of view correctly.

#### Aspect Ratio
The aspect ratio of the sensor is:

\[
\text{Aspect Ratio} = \frac{\text{Width}}{\text{Height}} = \frac{1920}{1080} = \frac{16}{9} \approx 1.7778
\]

Since the vertical FOV is 90°, the horizontal FOV can be calculated as:

\[
\text{Horizontal FOV} = 2 \cdot \arctan\left(\tan\left(\frac{\text{Vertical FOV}}{2}\right) \cdot \text{Aspect Ratio}\right)
\]
\[
\text{Horizontal FOV} = 2 \cdot \arctan\left(\tan\left(45°\right) \cdot \frac{16}{9}\right)
\]
\[
\text{Horizontal FOV} = 2 \cdot \arctan\left(1 \cdot \frac{16}{9}\right) \approx 2 \cdot \arctan(1.7778) \approx 2 \cdot 60.64° \approx 121.28°
\]

In NDC, for a vertical FOV of 90°, the Y-coordinate maps directly from \(-1\) to \(1\) to the height of the sensor (1080 pixels). However, for the X-coordinate, since the horizontal FOV is wider, the range \(-1\) to \(1\) in X corresponds to a wider field, but we still map it directly to the sensor width for simplicity, assuming the projection matrix accounts for the aspect ratio.

The mapping from NDC to pixel coordinates is:

- For X: \(X_{\text{pixel}} = \frac{\text{Width}}{2} \cdot (X' + 1)\)
- For Y: \(Y_{\text{pixel}} = \frac{\text{Height}}{2} \cdot (1 - Y')\) (flipping Y because pixel coordinates typically have the origin at the top-left)

In many graphics pipelines, pixel coordinates have \((0, 0)\) at the top-left, so we adjust the Y-coordinate accordingly.

First, let's compute the pixel coordinates assuming NDC maps directly to the sensor dimensions (with aspect ratio handled by the projection):

- **Width mapping**: \(X' = -1\) maps to pixel 0, \(X' = 1\) maps to pixel 1920.
\[
X_{\text{pixel}} = \frac{1920}{2} \cdot (X' + 1) = 960 \cdot (X' + 1)
\]

- **Height mapping**: \(Y' = -1\) maps to pixel 1080 (bottom), \(Y' = 1\) maps to pixel 0 (top).
\[
Y_{\text{pixel}} = \frac{1080}{2} \cdot (1 - Y') = 540 \cdot (1 - Y')
\]

Now substitute \(X' = -0.6667\), \(Y' = -0.3333\):

\[
X_{\text{pixel}} = 960 \cdot (-0.6667 + 1) = 960 \cdot 0.3333 \approx 320
\]
\[
Y_{\text{pixel}} = 540 \cdot (1 - (-0.3333)) = 540 \cdot 1.3333 \approx 720
\]

So, the point projects to approximately pixel \((320, 720)\).

---

### Step 4: Verify the Mapping
To ensure correctness, note that since the vertical FOV is 90°, the Y mapping should be straightforward (NDC Y from \(-1\) to \(1\) maps to 0 to 1080 pixels). For X, the wider horizontal FOV (121.28°) means that the full width of the sensor corresponds to a larger range in NDC if the aspect ratio were not accounted for. However, in standard perspective projection, the projection matrix typically adjusts for the aspect ratio, so the NDC range \(-1\) to \(1\) maps directly to the sensor dimensions.

If the problem intended a different FOV interpretation (e.g., horizontal FOV of 90°), the calculations would differ, but since it's unspecified and 90° FOV with a 16:9 aspect ratio is often vertical, we'll stick with this interpretation.

---

### Final Answer
The point \((2, 1, -3)\) in camera space projects to approximately pixel **(320, 720)** on a \(1920 \times 1080\) sensor.

