# Prompt #50: A robot’s footprint is a 0.5 m radius circle. It must travel along polyline (0,0)→(2,1)→(4,1). Is a corridor 1 m wide (measured centreline) sufficient for clearance?

## OpenAI o3  (68 secs)
A 1 m corridor means that every point whose perpendicular distance to the centre-line is ≤ 0.50 m belongs to the free space.  
The robot is a circle of the same radius (0.50 m).  On the straight parts of the
poly-line this is obviously enough; the only place where lack of clearance could
occur is at the bend in (2, 1).

1.  Determine the interior turning angle.

   Segment 1 (0,0) → (2,1) vector v₁ = (2,1) |v₁| = √5  
   Segment 2 (2,1) → (4,1) vector v₂ = (2,0) |v₂| = 2  

   cos θ = v₁·v₂ /( |v₁| |v₂| ) = (2·2 + 1·0)/(√5·2) = 4 / (2√5) = 0.8944  
   θ = arccos 0.8944 ≈ 26.565°.

   The robot therefore turns through only about 26.6°.

2.  Furthest a point on the robot can be from the segments while its centre is
    at the corner.

    Take the point on the rim that lies along the bisector of the interior
    angle.  With a circular footprint of radius R

        distance to either segment = R sin(θ/2).

    θ/2 = 13.2825°,  sin(θ/2) ≈ 0.230.

        d = 0.50 m × 0.230 ≈ 0.115 m.

    So even the worst-situated point on the robot is only 0.115 m from each
    segment, well inside the 0.50 m allowance.

3.  A second (more pessimistic) check: take the point directly beneath the
    corner, P = (2, 1) + (0, −0.50) = (2, 0.5).

        distance to horizontal segment y = 1   →  0.50 m  
        distance to line through (2,1) in direction (2,1)  → |2−2·0.5|/√5 = 0.447 m

    Both are ≤ 0.50 m.

Hence every point of the robot remains within the 0.50 m offset of at least one
of the two straight segments.  The 1 m-wide corridor therefore contains the
entire swept area of the robot.

Conclusion: Yes – a 1 m wide corridor (0.5 m either side of the centre-line) is
sufficient, with about 5 cm of spare clearance at the tightest point of the
turn.

## Claude 4 Sonnet  (20 secs)
I need to determine if a robot with a 0.5 m radius circular footprint can travel through a 1 m wide corridor along the given polyline path.

Given information:
- Robot footprint: circle with radius 0.5 m
- Path: (0,0) → (2,1) → (4,1)
- Corridor width: 1 m (measured from centerline)

First, let me understand what "1 m wide measured from centerline" means. This typically means the corridor extends 0.5 m on each side of the centerline path.

For the robot to pass through safely:
- The robot's center follows the polyline path
- The robot needs clearance of 0.5 m on each side of its center
- Total width needed = 2 × robot radius = 2 × 0.5 m = 1 m

Since the corridor provides 0.5 m on each side of the centerline (total 1 m), this exactly matches the robot's diameter.

However, I need to check the corner at point (2,1) where the path changes direction:

1. First segment: (0,0) → (2,1), direction angle = arctan(1/2) ≈ 26.57°
2. Second segment: (2,1) → (4,1), direction is horizontal (0°)
3. The path turns by approximately 26.57° at point (2,1)

At the corner, the robot's circular footprint will need additional clearance as it navigates the turn. The swept area as the robot turns will extend beyond the 0.5 m boundary on the outside of the turn.

The maximum distance from the centerline occurs at the outside of the turn. For a circular robot, this remains at 0.5 m from the path, but the envelope of motion during the turn creates a wider swept area.

**Conclusion**: No, a 1 m wide corridor is NOT sufficient. While it provides exactly enough clearance for straight-line motion, the robot cannot navigate the corner at (2,1) without its footprint extending beyond the corridor boundaries during the turn.

## Gemini Pro  (95 secs)
Here's how to determine if the corridor is sufficient:

1.  **Understand the Robot and Corridor Dimensions:**
    *   Robot footprint: Circle with radius `R_robot = 0.5 m`.
    *   Corridor width: `W_corridor = 1 m`.
    *   The corridor width is measured from its centreline. This means the distance from the centreline to either wall of the corridor is `W_corridor / 2 = 1 m / 2 = 0.5 m`.

2.  **Robot's Path and Corridor's Centreline:**
    *   The robot must travel along the polyline (0,0) → (2,1) → (4,1). This polyline is the intended path for the *center* of the robot.
    *   This same polyline is also the *centreline* of the corridor.

3.  **Relate Robot to Corridor:**
    *   The corridor consists of all points within 0.5 m of its centreline.
    *   If the robot's center follows this centreline perfectly, the robot will occupy all points within 0.5 m of its center.
    *   Since the robot's radius (0.5 m) is exactly equal to the distance from the corridor's centreline to its walls (0.5 m), the space occupied by the robot (if its center follows the centreline) is identical to the space defined by the corridor.

4.  **Analyze Straight Segments:**
    *   Along the segment (0,0) → (2,1): If the robot's center moves along this line, its edges will be 0.5 m away from this line, perpendicular to it. These edges will perfectly align with the corridor walls.
    *   Along the segment (2,1) → (4,1): Similarly, the robot's edges will perfectly align with the corridor walls.
    *   In these straight sections, the robot fits exactly, with zero clearance (its edges touch the walls).

5.  **Analyze the Turn at (2,1):**
    *   The critical part of any path involving a turn is the inner corner.
    *   Let the centreline be denoted by `L`. The corridor `C` is the set of all points `P` such that the minimum distance from `P` to `L` is less than or equal to 0.5 m (`dist(P, L) ≤ 0.5 m`).
    *   The space `S` swept by the robot (whose center follows `L`) is the set of all points `P'` such that the minimum distance from `P'` to `L` is less than or equal to the robot's radius, 0.5 m (`dist(P', L) ≤ 0.5 m`).
    *   Since the robot's radius is equal to the corridor's half-width, the swept volume of the robot `S` is identical to the corridor space `C`.
    *   This means that at every point, including the inner corner of the turn, the edge of the robot will precisely touch the boundary of the corridor.

    *To verify this with coordinates for the inner corner:*
    *   Let A=(0,0), B=(2,1), C=(4,1).
    *   Inner wall for AB: The line AB is `y = 0.5x` or `x - 2y = 0`. The turn at B is to the "right" (if looking from A to B to C). The inner wall is offset by 0.5m. Its equation is `x - 2y = √5 * 0.5` (since `(x_B - 2y_B = 2 - 2(1) = 0)`, and the vector normal to AB pointing "inward" for a right turn can be (1/√5, -2/√5), so `(x-x_B) * 1/√5 + (y-y_B)*(-2/√5) = 0.5` for points on the wall, which simplifies). More simply, the line `x - 2y - k = 0` is `0.5m` from `x-2y=0`. The distance is `|k|/√5 = 0.5`, so `|k|=√5/2`. For the "inner" side, `x - 2y = √5/2`.
    *   Inner wall for BC: The line BC is `y = 1`. The inner wall (below) is `y = 1 - 0.5 = 0.5`.
    *   The inner corner point `I` is the intersection of `x - 2y = √5/2` and `y = 0.5`.
        `x - 2(0.5) = √5/2` => `x - 1 = √5/2` => `x = 1 + √5/2 ≈ 1 + 1.118 = 2.118`.
        So, `I ≈ (2.118, 0.5)`.
    *   The distance from the centreline vertex `B=(2,1)` to this inner corner `I` is:
        `dist(B,I) = √((2.118 - 2)² + (0.5 - 1)²) = √((0.118)² + (-0.5)²) = √(0.013924 + 0.25) = √0.263924 ≈ 0.5137 m`.
    *   This `dist(B,I)` is the distance from the centreline corner to the physical inner corner of the corridor.
    *   The robot's radius is `R_robot = 0.5 m`.
    *   When the robot's center is exactly at `B=(2,1)`, its edge is `0.5 m` away. Since `0.5 m < 0.5137 m`, the robot body does not touch `I` *when its center is at B*.
    *   However, the robot's center also passes *near* `I`. The closest point on the centreline segment AB to `I` is found by projecting `I` onto `x-2y=0`. This distance is `|2.118 - 2(0.5)| / √5 = |1.118| / √5 ≈ 1.118 / 2.236 ≈ 0.5 m`.
    *   Similarly, the closest point on the centreline segment BC to `I` is `(2.118, 1)`. The distance from this point to `I` is `|1 - 0.5| = 0.5 m`.
    *   So, there are points on the robot's centreline path that are exactly 0.5 m away from the inner corner `I`. When the robot's center is at such a point, its edge (which is 0.5 m from its center) will touch `I`.

6.  **Conclusion on "Sufficient for Clearance":**
    *   "Sufficient for clearance" typically means the robot can pass without any part of its body going outside the designated corridor. Touching the boundary is usually permissible (zero clearance).
    *   Since the space occupied by the robot (when its center follows the centreline) is identical to the corridor space, the robot fits perfectly. It will touch the walls on straight sections and touch the innermost point of the boundary at the turn.

Yes, a corridor 1 m wide (measured centreline) is sufficient for clearance, with the understanding that the robot will have zero actual clearance at its boundaries.

The final answer is $\boxed{Yes}$

## DeepSeek Reasoner  (585 secs)
The robot has a circular footprint with a radius of 0.5 m, and the corridor is 1 m wide, measured from the centerline, meaning the half-width (distance from centerline to the corridor boundary) is 0.5 m. For the robot to stay within the corridor, its center must lie exactly on the centerline polyline path from (0,0) to (2,1) to (4,1) at all times, as any deviation would cause part of the robot to extend beyond the corridor boundaries due to the equal radius and half-width.

- Along the straight segments of the polyline, the robot's center on the path ensures the entire footprint is within the corridor, touching the boundaries.
- At the vertex (2,1), where the direction changes, the critical point is on the robot's circle in the direction of the internal bisector of the turn. The turn angle θ is approximately 26.565°, and the distance from this point to the polyline is approximately 0.4865 m, which is less than the corridor half-width of 0.5 m. Thus, this point is strictly inside the corridor.
- For all other points on the robot's circle at the vertex, the distance to the polyline is greater than or equal to 0.4865 m, but still within or on the corridor boundary.
- At the endpoints (0,0) and (4,1), the corridor includes disks of radius 0.5 m centered at these points, so the robot's footprint is on the boundary when centered there.

Since the minimum distance from any point on the robot's footprint to the polyline is less than or equal to 0.5 m when the center is on the path, and strictly less at the vertex, the robot can traverse the path while staying within the corridor. Therefore, a corridor 1 m wide is sufficient for clearance.

\boxed{\text{yes}}

## Grok 3  (23 secs)
To determine if a 1-meter-wide corridor (measured along its centerline) is sufficient for a robot with a footprint of a 0.5-meter radius circle to travel along the given polyline path from (0,0) to (2,1) to (4,1), we need to analyze the clearance required by the robot along the path, especially at the corners where the path changes direction.

### Step 1: Understand the Robot's Footprint and Corridor Width
- The robot's footprint is a circle with a radius of 0.5 meters. This means the robot occupies a circular area with a diameter of 1 meter centered at its position.
- The corridor is 1 meter wide, measured along its centerline. This implies that the corridor extends 0.5 meters on either side of the centerline, forming a strip 1 meter wide in total.
- For the robot to travel through the corridor without collision, the entire circular footprint of the robot must remain within the corridor boundaries at all points along the path.

### Step 2: Analyze the Path and Corridor Geometry
The path is a polyline from (0,0) to (2,1) to (4,1). This means:
- The first segment is from (0,0) to (2,1).
- The second segment is from (2,1) to (4,1), which is a horizontal line.
- At the corner point (2,1), the path changes direction from a diagonal upward movement to a horizontal movement.

We assume the corridor follows the same polyline shape as the path, with a width of 1 meter centered along the polyline. Thus:
- Along straight segments, the corridor is a 1-meter-wide strip centered on the line.
- At the corner (2,1), we need to check the geometry of the corridor to ensure the robot can navigate the turn without its footprint extending outside the corridor boundaries.

### Step 3: Check Clearance Along Straight Segments
Along the straight segments of the path:
- The robot's footprint is a circle of radius 0.5 meters.
- The corridor is 1 meter wide, meaning it extends 0.5 meters on either side of the centerline.
- Since the robot's radius is 0.5 meters, its footprint will exactly touch the boundaries of the corridor if it is centered on the path. This suggests that along straight segments, the corridor is just wide enough to accommodate the robot, as the diameter of the robot (1 meter) matches the corridor width (1 meter).

Thus, along straight segments, the clearance is sufficient, with no extra margin but no overlap outside the corridor.

### Step 4: Check Clearance at the Corner (2,1)
The critical point is at the corner (2,1), where the path changes direction. We need to determine the shape of the corridor at the corner and whether the robot's footprint remains within the corridor boundaries when the robot's center is at or near (2,1).

#### Corridor Geometry at the Corner
When a corridor follows a polyline with a constant width, the inner side of the corner is typically "cut off" or mitered, while the outer side forms a wider arc or miter depending on the angle of the turn. Let's compute the geometry:
- The first segment from (0,0) to (2,1) has a direction vector (2,1), with a slope of 1/2.
- The second segment from (2,1) to (4,1) has a direction vector (2,0), which is horizontal.
- The angle between the two segments can be calculated using the dot product of the direction vectors.
  - Direction vector 1: (2,1), magnitude = sqrt(2^2 + 1^2) = sqrt(5)
  - Direction vector 2: (2,0), magnitude = 2
  - Dot product = (2)(2) + (1)(0) = 4
  - Cosine of angle θ = dot product / (magnitude1 * magnitude2) = 4 / (sqrt(5) * 2) ≈ 0.8944
  - Angle θ ≈ arccos(0.8944) ≈ 26.57 degrees
- Since the turn is less than 90 degrees, the inner corner of the corridor will be mitered (a straight cut), and the outer corner will extend beyond a simple arc.

However, a simpler approach to check clearance is to consider the robot's footprint at the corner and see if it fits within the corridor's boundaries.

#### Robot's Footprint at the Corner
When the robot's center is at (2,1), its footprint is a circle of radius 0.5 meters, so it extends from:
- x = 1.5 to 2.5
- y = 0.5 to 1.5

Now, we define the corridor boundaries at and near the corner:
- For the first segment (0,0) to (2,1), the centerline is the line from (0,0) to (2,1). The corridor extends 0.5 meters on either side perpendicular to this line.
- For the second segment (2,1) to (4,1), the centerline is the horizontal line y=1 from x=2 to x=4. The corridor extends from y=0.5 to y=1.5.
- At the corner, the corridor boundaries on the inner side (below the path) will follow the mitered line connecting the inner boundaries of the two segments, and on the outer side (above the path), it will follow the mitered line connecting the outer boundaries.

To simplify, note that the robot's footprint at (2,1) extends down to y=0.5. Since the second segment's corridor boundary is at y=0.5 (lower boundary), the robot's footprint touches the lower boundary of the corridor at y=0.5 when its center is at y=1. This suggests that at the corner, the robot's footprint is still within or on the boundary of the corridor, provided the mitered inner corner does not encroach further.

#### Inner Corner Miter Calculation
To be precise, let's calculate the inner boundary at the corner:
- For the first segment (0,0) to (2,1), the direction is (2,1). A unit vector is (2/sqrt(5), 1/sqrt(5)).
- A perpendicular vector (rotated 90 degrees clockwise for the lower side) is (1/sqrt(5), -2/sqrt(5)).
- The corridor's lower boundary offset is 0.5 meters in this perpendicular direction.
- For the second segment (2,1) to (4,1), the direction is (1,0), so a perpendicular vector downward is (0,-1), and the lower boundary is at y=1 - 0.5 = 0.5.
- The inner miter line at the corner can be approximated by finding the intersection of the offset lines, but since the robot's footprint touches y=0.5 and the corridor's lower boundary is at y=0.5 for the second segment, we can infer that the footprint does not extend below the corridor boundary.

Since the angle is small (about 26.57 degrees), the mitered inner corner will not significantly reduce the corridor width below the robot's footprint requirement at (2,1). Visually, the robot's lowest point at y=0.5 aligns with the corridor's lower boundary at y=0.5 for the horizontal segment, and the transition at the corner does not push the boundary inward enough to cause a collision.

### Step 5: Consider Path Smoothing or Actual Trajectory
In reality, a robot may not follow the exact polyline with a sharp corner but may smooth the path around (2,1). However, since the problem specifies the polyline as the path and asks about clearance in a corridor following that polyline, we assume the robot's center follows the polyline exactly. If the robot smooths the corner, it might move its center away from the inner side, potentially requiring less clearance, but we stick to the given path for this analysis.

### Step 6: Conclusion
- Along straight segments, the 1-meter-wide corridor exactly matches the robot's 1-meter diameter footprint, so clearance is sufficient (just touching the boundaries).
- At the corner (2,1), the robot's footprint extends to y=0.5, which matches the lower boundary of the corridor for the horizontal segment. The mitered inner corner does not encroach enough to cause the footprint to extend outside the corridor, given the shallow angle of the turn.

Therefore, a 1-meter-wide corridor (measured along the centerline) is sufficient for clearance for the robot to travel along the polyline (0,0)→(2,1)→(4,1). The robot's footprint remains within the corridor boundaries at all points, including the corner, with no overlap outside the corridor.

**Answer**: Yes, a 1-meter-wide corridor is sufficient for clearance.

