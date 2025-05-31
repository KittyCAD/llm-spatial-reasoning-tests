# Prompt #3: Slice a cube with a plane that passes through the mid-points of three mutually adjacent edges. What shape is the cross-section?

## OpenAI o3
Label the cube so that the three mutually–adjacent edges issue from the vertex O and lie on the positive coordinate axes:

O = (0, 0, 0)  
A = (1, 0, 0) (edge OA)  
B = (0, 1, 0) (edge OB)  
C = (0, 0, 1) (edge OC)

(The cube occupies 0 ≤ x, y, z ≤ 1.)

Mid-points of those three edges are  
P = (½, 0, 0), Q = (0, ½, 0), R = (0, 0, ½).

1.  Plane through P, Q, R  
   Vectors PQ = (–½, ½, 0) and PR = (–½, 0, ½)  
   ⇒ normal vector n = PQ × PR = (1, 1, 1).  
   Hence the plane equation is
         x + y + z = ½.

2.  Where does this plane meet the cube?  
    Inside the cube we have 0 ≤ x, y, z ≤ 1, so x + y + z ≤ 3.  
    Because the plane requires x + y + z = ½, every point of intersection
    satisfies x, y, z ≤ ½; consequently the only edges reached are the
    three whose mid-points we already have.  
    Therefore the cross-section of the cube with this plane is the
    triangle ΔPQR.

3.  Shape of ΔPQR  
        |PQ|² = (½)² + (½)² = ½  
        |QR|² = (½)² + (½)² = ½  
        |RP|² = (½)² + (½)² = ½  

    All three sides are equal, so ΔPQR is equilateral.

Hence the plane cuts the cube in an equilateral triangle.

## Claude 4 Sonnet
I need to find the shape of the cross-section when a plane passes through the midpoints of three mutually adjacent edges of a cube.

Let me set up a coordinate system. I'll place a unit cube with vertices at coordinates where each coordinate is either 0 or 1.

First, I need to identify three mutually adjacent edges. Three edges are mutually adjacent if they share a common vertex. Let me choose the vertex at the origin (0,0,0). The three edges from this vertex go to:
- (1,0,0) 
- (0,1,0)
- (0,0,1)

The midpoints of these three edges are:
- M₁ = (1/2, 0, 0) - midpoint of edge from (0,0,0) to (1,0,0)
- M₂ = (0, 1/2, 0) - midpoint of edge from (0,0,0) to (0,1,0)  
- M₃ = (0, 0, 1/2) - midpoint of edge from (0,0,0) to (0,0,1)

Now I need to find the equation of the plane passing through these three points.

The vectors from M₁ to M₂ and from M₁ to M₃ are:
- v₁ = M₂ - M₁ = (-1/2, 1/2, 0)
- v₂ = M₃ - M₁ = (-1/2, 0, 1/2)

The normal vector to the plane is:
n = v₁ × v₂ = (1/4, 1/4, 1/4) = (1/4)(1, 1, 1)

So the plane equation is x + y + z = d, where d can be found by substituting any of the three points:
1/2 + 0 + 0 = 1/2

Therefore, the plane equation is: x + y + z = 1/2

Now I need to find where this plane intersects the cube. The cube has vertices at all points (x,y,z) where each coordinate is 0 or 1. The plane x + y + z = 1/2 will intersect:

1. The three midpoints we started with: (1/2,0,0), (0,1/2,0), (0,0,1/2)

2. The edges of the cube where the plane crosses them. Since x + y + z = 1/2 and each coordinate is between 0 and 1, the plane will also intersect three more edges at their midpoints:
   - (0,1,0) to (0,1,1): intersection at (0,1/2,0) - wait, this is M₂
   - Actually, I need to check all edges systematically...

Let me reconsider. The plane x + y + z = 1/2 intersects the cube at points where:
- Three vertices of the intersection are the three midpoints we identified
- We need to find the other vertices

By symmetry and the nature of the plane equation, the plane will also pass through:
- The midpoint of the edge from (1,1,0) to (1,1,1): point (1,1,1/2) ❌ No, 1+1+1/2 = 5/2 ≠ 1/2
- Let me check systematically...

The plane intersects edges where one coordinate varies from 0 to 1 while the others are fixed. For x + y + z = 1/2:
- Edge from (0,0,1) to (0,1,1): at (0,0,1) we get 0+0+1=1 and at (0,1,1) we get 0+1+1=2, so no intersection
- Edge from (0,1,0) to (1,1,0): intersection where x+1+0=1/2, so x=-1/2 ❌ (outside cube)
- Edge from (1,0,0) to (1,0,1): intersection where 1+0+z=1/2, so z=-1/2 ❌ (outside cube)

Actually, the plane must intersect exactly 6 

## Gemini Pro
Let the cube have vertices at (0,0,0), (s,0,0), (0,s,0), (0,0,s), (s,s,0), (s,0,s), (0,s,s), (s,s,s), where 's' is the side length of the cube.

Consider the three mutually adjacent edges meeting at the origin (0,0,0):
1.  The edge along the x-axis: from (0,0,0) to (s,0,0)
2.  The edge along the y-axis: from (0,0,0) to (0,s,0)
3.  The edge along the z-axis: from (0,0,0) to (0,0,s)

The mid-points of these three edges are:
*   P1: (s/2, 0, 0)
*   P2: (0, s/2, 0)
*   P3: (0, 0, s/2)

The plane passes through these three points P1, P2, and P3. The cross-section will be the triangle formed by connecting these three points. Let's find the lengths of the sides of this triangle:

Length of P1P2:
Using the distance formula, d = sqrt((x2-x1)^2 + (y2-y1)^2 + (z2-z1)^2)
d(P1,P2) = sqrt((0 - s/2)^2 + (s/2 - 0)^2 + (0 - 0)^2)
= sqrt((-s/2)^2 + (s/2)^2 + 0)
= sqrt(s^2/4 + s^2/4)
= sqrt(2s^2/4)
= sqrt(s^2/2) = s/√2 = (s√2)/2

Length of P2P3:
d(P2,P3) = sqrt((0 - 0)^2 + (0 - s/2)^2 + (s/2 - 0)^2)
= sqrt(0 + (-s/2)^2 + (s/2)^2)
= sqrt(s^2/4 + s^2/4)
= sqrt(2s^2/4)
= sqrt(s^2/2) = s/√2 = (s√2)/2

Length of P3P1:
d(P3,P1) = sqrt((s/2 - 0)^2 + (0 - 0)^2 + (0 - s/2)^2)
= sqrt((s/2)^2 + 0 + (-s/2)^2)
= sqrt(s^2/4 + s^2/4)
= sqrt(2s^2/4)
= sqrt(s^2/2) = s/√2 = (s√2)/2

Since all three sides of the triangle P1P2P3 have the same length ((s√2)/2), the triangle is an **equilateral triangle**.

Visually, the plane slices off a small corner of the cube. The three points lie on three different faces of the cube that meet at the chosen corner. The segments connecting these midpoints lie on these faces.

The shape of the cross-section is an **equilateral triangle**.

## DeepSeek Reasoner
To determine the cross-section shape when a cube is sliced by a plane passing through the midpoints of three mutually adjacent edges, consider a cube with side length \(a\). Position the cube in 3D space with coordinates such that one vertex is at the origin \((0, 0, 0)\), and the adjacent vertices are at \((a, 0, 0)\), \((0, a, 0)\), and \((0, 0, a)\). The three mutually adjacent edges are those from \((0, 0, 0)\) to \((a, 0, 0)\), from \((0, 0, 0)\) to \((0, a, 0)\), and from \((0, 0, 0)\) to \((0, 0, a)\).

The midpoints of these edges are:
- Midpoint of edge to \((a, 0, 0)\): \(A(a/2, 0, 0)\)
- Midpoint of edge to \((0, a, 0)\): \(B(0, a/2, 0)\)
- Midpoint of edge to \((0, 0, a)\): \(C(0, 0, a/2)\)

The plane passing through these points has the equation \(x + y + z = a/2\).

The intersection of this plane with the cube is found by determining where the plane meets the cube's faces. The plane intersects:
- The face \(z = 0\) (bottom face) along the line segment from \(A(a/2, 0, 0)\) to \(B(0, a/2, 0)\).
- The face \(y = 0\) (front face, assuming \(y\) is the depth) along the line segment from \(A(a/2, 0, 0)\) to \(C(0, 0, a/2)\).
- The face \(x = 0\) (left face) along the line segment from \(B(0, a/2, 0)\) to \(C(0, 0, a/2)\).

No intersections occur with the faces \(x = a\), \(y = a\), or \(z = a\) because the equations yield values outside the cube's bounds (e.g., on \(x = a\), \(a + y + z = a/2\) implies \(y + z = -a/2\), which is impossible for \(y, z \geq 0\)).

The line segments \(AB\), \(AC\), and \(BC\) form a triangle with vertices at \(A\), \(B\), and \(C\). Calculating the distances:
- \(AB = \sqrt{(a/2 - 0)^2 + (0 - a/2)^2 + (0 - 0)^2} = \sqrt{(a/2)^2 + (-a/2)^2} = \sqrt{a^2/4 + a^2/4} = \sqrt{a^2/2} = a/\sqrt{2}\)
- \(AC = \sqrt{(a/2 - 0)^2 + (0 - 0)^2 + (0 - a/2)^2} = \sqrt{(a/2)^2 + (-a/2)^2} = \sqrt{a^2/4 + a^2/4} = \sqrt{a^2/2} = a/\sqrt{2}\)
- \(BC = \sqrt{(0 - 0)^2 + (a/2 - 0)^2 + (0 - a/2)^2} = \sqrt{(a/2)^2 + (-a/2)^2} = \sqrt{a^2/4 + a^2/4} = \sqrt{a^2/2} = a/\sqrt{2}\)

All sides are equal, confirming that the triangle is equilateral. The cross-section is entirely within the cube and bounded by these segments, with no other intersection points.

Thus, the cross-section is an equilateral triangle with side length \(a/\sqrt{2}\).

\boxed{\text{equilateral triangle}}

## Grok 3
To determine the shape of the cross-section when a cube is sliced by a plane passing through the midpoints of three mutually adjacent edges, let’s visualize the cube and define the plane systematically.

Consider a cube with vertices labeled as follows in a 3D coordinate system (assuming the cube has side length 2 for simplicity, centered at the origin):
- A: (-1, -1, -1)
- B: (1, -1, -1)
- C: (1, 1, -1)
- D: (-1, 1, -1)
- E: (-1, -1, 1)
- F: (1, -1, 1)
- G: (1, 1, 1)
- H: (-1, 1, 1)

The edges of the cube connect these vertices. "Mutually adjacent edges" means three edges that share a common vertex (i.e., they form a corner of the cube). Let's choose the vertex at A (-1, -1, -1) and the three edges emanating from it:
- Edge AB: from A (-1, -1, -1) to B (1, -1, -1)
- Edge AD: from A (-1, -1, -1) to D (-1, 1, -1)
- Edge AE: from A (-1, -1, -1) to E (-1, -1, 1)

Now, find the midpoints of these edges:
- Midpoint of AB: ((-1+1)/2, (-1-1)/2, (-1-1)/2) = (0, -1, -1)
- Midpoint of AD: ((-1-1)/2, (-1+1)/2, (-1-1)/2) = (-1, 0, -1)
- Midpoint of AE: ((-1-1)/2, (-1-1)/2, (-1+1)/2) = (-1, -1, 0)

So, the plane passes through the points:
- P: (0, -1, -1)
- Q: (-1, 0, -1)
- R: (-1, -1, 0)

To confirm that these points define a plane, we can find the equation of the plane. The general equation of a plane is ax + by + cz = d. Using the three points, we solve for the coefficients.

First, find two vectors in the plane:
- Vector PQ: Q - P = (-1 - 0, 0 - (-1), -1 - (-1)) = (-1, 1, 0)
- Vector PR: R - P = (-1 - 0, -1 - (-1), 0 - (-1)) = (-1, 0, 1)

The normal vector to the plane is the cross product of PQ and PR:
PQ × PR = |i    j    k|
          |-1   1    0|
          |-1   0    1|

= i(1*1 - 0*0) - j((-1)*1 - (-1)*0) + k((-1)*0 - (-1)*1)
= i(1) - j(-1) + k(1)
= (1, 1, 1)

Thus, the normal vector is (1, 1, 1), and the plane equation is x + y + z = d. Plugging in point P (0, -1, -1):
0 + (-1) + (-1) = -2
So, the plane equation is x + y + z = -2.

Now, we find the intersection of this plane with the cube. The cube’s boundaries are from x = -1 to 1, y = -1 to 1, z = -1 to 1. We need to find the points where the plane intersects the edges of the cube.

Since the plane passes through the midpoints of edges, and due to the symmetry of the cube, we can anticipate that the cross-section will be a polygon. Let’s find the vertices of the cross-section by checking intersections with all edges.

However, notice that the plane x + y + z = -2 is symmetric, and since the cube is symmetric, the plane will cut through midpoints of edges in a balanced way. Due to the choice of three midpoints around one corner, the plane will intersect three edges at their midpoints as given, and it will also intersect the opposite edges or faces.

Let me consider the opposite corner of the cube, which is G (1, 1, 1). Check if the plane passes through midpoints of edges near G.

Edges from G:
- GB: (1,1,1) to (1,-1,1), midpoint (1,0,1)
- GC: (1,1,1) to (1,1,-1), midpoint (1,1,0)
- GH: (1,1,1) to (-1,1,1), midpoint (0,1,1)

Check if these satisfy x + y + z = -2:
- (1,0,1): 1+0+1=2 ≠ -2
- (1,1,0): 1+1+0=2 ≠ -2
- (0,1,1): 0+1+1=2 ≠ -2

None of them lie on the plane. I may have miscalculated intersections. Let's solve for intersections of the plane with all 12 edges of the cube.

List the edges and find intersections:
1. AB: (-1,-1,-1) to (1,-1,-1), y=-1, z=-1, x + (-1) + (-1) = -2 → x-2=-2 → x=0. Point (0,-1,-1) [already have]
2. AD: (-1,-1,-1) to (-1,1,-1), x=-1, z=-1, -1 + y -1 = -2 → y-2=-2 → y=0. Point (-1,0,-1) [already have]
3. AE: (-1,-1,-1) to (-1,-1,1), x=-1, y=-1, -1 -1 + z = -2 → z-2=-2 → z=0. Point (-1,-1,0) [already have]
4. BC: (1,-1,-1) to (1,1,-1), x=1, z=-1, 1 + y -1 = -2 → y+0=-2 → y=-2. Outside cube (-2 < -1), no intersection.
5. BF: (1,-1,-1) to (1,-1,1), x=1, y=-1, 1 -1 + z = -2 → z+0=-2 → z=-2. Outside cube, no intersection.
6. DC: (-1,1,-1) to (1,1,-1), y=1, z=-1, x +1 -1 = -2 → x+0=-2 → x=-2. Outside cube, no intersection.
7. DH: (-1,1,-1) to (-1,1,1), x=-1, y=1, -1 +1 + z = -2 → z+0=-2 → z=-2. Outside cube, no intersection.
8. EF: (-1,-1,1) to (1,-1,1), y=-1, z=1, x -1 +1 = -2 → x+0=-2 → x=-2. Outside cube, no intersection.
9. EH: (-1,-1,1) to (-1,1,1), x=-1, z=1, -1 + y +1 = -2 → y+0=-2 → y=-2. Outside cube, no intersection.
10. FC: (1,-1,1) to (1,1,1), x=1, z=1, 1 + y +1 = -2 → y+2=-2 → y=-4. Outside cube, no intersection.
11. FG: (1,-1,1) to (1,1,-1), x=1, 1 + y + z = -2 → y + z = -3. y from -1 to 1, z from -1 to 1, y+z from -2 to 2, includes -3? y=1, z=-4 (out); y=-1, z=-2 (out). Compute: y=-1, z= -3-1=-4 (out); y=1, z=-3-1=-4 (out). No intersection within cube.
12. HG: (-1,1,1) to (1,1,-1), y=1, x +1 + z = -2 → x + z = -3. x from -1 to 1, z from -1 to 1, x+z from -2 to 2, includes -3? x=-1, z=-2 (out); x=1, z=-4 (out). No intersection.

I seem to have only three intersection points: (0,-1,-1), (-1,0,-1), (-1,-1,0). That suggests a triangle, but I suspect I missed intersections because a plane cutting through a cube should intersect more edges if it's not aligned with faces.

I may have made a mistake in assuming the cube's orientation or coordinates. Let me reconsider the plane's intersections. Since the plane x+y+z=-2, and cube ranges from -1 to 1, the sum x+y+z ranges from -3 to 3, so -2 is within range.

Test points on other edges or reconsider:
Consider edges at the opposite corner or other faces.

Edges from F (1,-1,1):
- FB: (1,-1,1) to (1,-1,-1): already checked, no intersection.
- FC: (1,-1,1) to (1,1,1): x=1, y from -1 to 1, z from 1 to 1 (mistake earlier, z from -1 to 1 in earlier list but correctly it's to G(1,1,1) wait no: F to C is (1,-1,1) to (1,1,-1), y=-1 to 1, z=1 to -1.
So x=1, y + z = -3, y from -1 to 1, z= -3-y, y=-1, z=-2 (out); y=1, z=-4 (out). Still no.

I think I need a better way. Since it's a plane cutting a cube, and I have three points, I can see the shape directly.

Plot the points I have:
- (0,-1,-1) on edge AB
- (-1,0,-1) on edge AD
- (-1,-1,0) on edge AE

These three points form a triangle, as they are midpoints of edges from one corner.

In a cube, a plane cutting through three midpoints of edges from one vertex should form a triangle, because the plane is "chopping off" the corner of the cube, and the intersection should be a triangular face.

Visualize: if I take a cube and cut near one corner through the midpoints of the three edges meeting at that corner, the plane should intersect only those three edges at their midpoints, forming a triangle, because the plane tilts away from the rest of the cube.

Confirm with the plane equation x+y+z=-2. At the opposite corner (1,1,1), x+y+z=3, which is greater than -2, so the plane does not cut through the opposite side; it only cuts near the corner A.

Check another point, say on edge HG or others, but earlier calculations show no other intersections within the cube boundaries.

Thus, the cross-section is indeed a triangle with vertices at (0,-1,-1), (-1,0,-1), and (-1,-1,0).

Therefore, the shape of the cross-section is a **triangle**.

