# Prompt #24: A right-triangle has legs 9 m and 12 m. A circle is inscribed (touching all three sides). What is the circle’s radius?

## OpenAI o3
For any triangle the inradius r is related to its area A and semiperimeter s by

  r = A / s.

1. Compute the sides:
   Legs: a = 9 m, b = 12 m  
   Hypotenuse: c = √(9² + 12²) = √225 = 15 m

2. Area of the right triangle:
   A = ½·a·b = ½·9·12 = 54 m²

3. Semiperimeter:
   s = (a + b + c) / 2 = (9 + 12 + 15) / 2 = 36 / 2 = 18 m

4. Inradius:
   r = A / s = 54 / 18 = 3 m

Therefore, the radius of the inscribed circle is 3 meters.

## Claude 4 Sonnet
I need to find the radius of the inscribed circle (incircle) of a right triangle with legs 9 m and 12 m.

First, let me find the hypotenuse using the Pythagorean theorem:
- $c^2 = a^2 + b^2$
- $c^2 = 9^2 + 12^2 = 81 + 144 = 225$
- $c = 15$ m

For a right triangle, there's a formula for the radius of the inscribed circle:
$$r = \frac{a + b - c}{2}$$

where $a$ and $b$ are the legs and $c$ is the hypotenuse.

Let me verify this formula makes sense. For any triangle, the radius of the inscribed circle is:
$$r = \frac{\text{Area}}{\text{Semi-perimeter}}$$

For our right triangle:
- Area = $\frac{1}{2} \times 9 \times 12 = 54$ m²
- Perimeter = $9 + 12 + 15 = 36$ m
- Semi-perimeter = $s = 18$ m

Therefore: $r = \frac{54}{18} = 3$ m

Let me verify using the formula for right triangles:
$$r = \frac{a + b - c}{2} = \frac{9 + 12 - 15}{2} = \frac{6}{2} = 3 \text{ m}$$

Both methods give the same answer.

The radius of the inscribed circle is **3 m**.

## Gemini Pro
Let the right-triangle be denoted as ABC, with the right angle at vertex C. Let the lengths of the legs be a and b, and the length of the hypotenuse be c.
Given:
Length of one leg, a = 9 m.
Length of the other leg, b = 12 m.

First, we need to find the length of the hypotenuse, c. According to the Pythagorean theorem for a right-triangle:
c² = a² + b²
c² = 9² + 12²
c² = 81 + 144
c² = 225
c = √225
c = 15 m.

Let r be the radius of the inscribed circle (also known as the inradius).
There are two common methods to find the inradius.

Method 1: Using the formula specific to right-triangles.
For a right-triangle with legs a and b and hypotenuse c, the radius of the inscribed circle is given by the formula:
r = (a + b - c) / 2

Substitute the values of a, b, and c into this formula:
r = (9 + 12 - 15) / 2
r = (21 - 15) / 2
r = 6 / 2
r = 3 m.

To understand where this formula comes from:
Let the triangle be ABC, with C as the right angle. Let the inscribed circle touch sides BC, AC, AB at points P, Q, R respectively. Let I be the incenter (center of the circle) and r be the radius.
IP ⊥ BC, IQ ⊥ AC, IR ⊥ AB. Also, IP = IQ = IR = r.
Consider the quadrilateral IQCP. Angle C is 90°. Angles IQC and IPC are also 90° (radius to tangent point). Thus, IQCP is a rectangle. Since IQ = IP = r, IQCP is a square with side length r.
So, CP = r and CQ = r.
The length of tangent segments from a vertex to the circle are equal:
AQ = AR. Since AC = b, AQ = AC - CQ = b - r. So, AR = b - r.
BP = BR. Since BC = a, BP = BC - CP = a - r. So, BR = a - r.
The hypotenuse AB = c = AR + RB.
Substituting the expressions for AR and RB:
c = (b - r) + (a - r)
c = a + b - 2r
Rearranging for r:
2r = a + b - c
r = (a + b - c) / 2.
This confirms the formula used.

Method 2: Using the general formula involving Area and semi-perimeter.
For any triangle, the radius of the inscribed circle is given by the formula:
r = K / s
where K is the area of the triangle, and s is the semi-perimeter of the triangle.

First, calculate the area K of the right-triangle:
K = (1/2) * base * height
K = (1/2) * a * b
K = (1/2) * 9 * 12
K = (1/2) * 108
K = 54 m².

Next, calculate the semi-perimeter s of the triangle:
s = (a + b + c) / 2
s = (9 + 12 + 15) / 2
s = 36 / 2
s = 18 m.

Now, calculate the inradius r:
r = K / s
r = 54 / 18
r = 3 m.

Both methods yield the same result.

A quick check using scaling:
The given triangle (9m, 12m, 15m) is a scaled version of the basic (3, 4, 5) right-triangle, scaled by a factor of 3 (since 9=3*3, 12=3*4, 15=3*5).
For a (3, 4, 5) triangle:
a=3, b=4, c=5.
The inradius would be r_basic = (3 + 4 - 5) / 2 = (7 - 5) / 2 = 2 / 2 = 1.
Since the triangle's sides are scaled by a factor of 3, the inradius should also be scaled by the same factor:
r = 3 * r_basic = 3 * 1 = 3 m.
This confirms the result again.

The final answer is $\boxed{3 \text{ m}}$.

## DeepSeek Reasoner
The right triangle has legs of 9 m and 12 m. The hypotenuse is calculated using the Pythagorean theorem:  
\[
\text{Hypotenuse} = \sqrt{9^2 + 12^2} = \sqrt{81 + 144} = \sqrt{225} = 15 \text{ m}.
\]

The radius \(r\) of the inscribed circle (incircle) that touches all three sides can be found using the formula for the inradius of a triangle:  
\[
r = \frac{A}{s},
\]  
where \(A\) is the area and \(s\) is the semi-perimeter.

The area \(A\) of a right triangle is given by:  
\[
A = \frac{1}{2} \times \text{leg}_1 \times \text{leg}_2 = \frac{1}{2} \times 9 \times 12 = 54 \text{ m}^2.
\]

The perimeter is the sum of the sides:  
\[
\text{Perimeter} = 9 + 12 + 15 = 36 \text{ m}.
\]  
The semi-perimeter \(s\) is half the perimeter:  
\[
s = \frac{36}{2} = 18 \text{ m}.
\]

Thus, the inradius is:  
\[
r = \frac{A}{s} = \frac{54}{18} = 3 \text{ m}.
\]

Alternatively, for a right triangle, the inradius can be found using the formula:  
\[
r = \frac{a + b - c}{2},
\]  
where \(a\) and \(b\) are the legs and \(c\) is the hypotenuse. Substituting the values:  
\[
r = \frac{9 + 12 - 15}{2} = \frac{6}{2} = 3 \text{ m}.
\]

Both methods confirm that the radius of the inscribed circle is 3 m.

\boxed{3}

## Grok 3
To find the radius of the circle inscribed in a right triangle with legs of 9 m and 12 m, we can use the formula for the inradius of a triangle. Let's go through the steps systematically.

First, since it's a right triangle, we can determine the length of the hypotenuse using the Pythagorean theorem. The legs are 9 m and 12 m, so:

\[
\text{Hypotenuse} = \sqrt{9^2 + 12^2} = \sqrt{81 + 144} = \sqrt{225} = 15 \, \text{m}
\]

Thus, the sides of the triangle are 9 m, 12 m, and 15 m.

Next, the formula for the inradius \( r \) of a triangle is:

\[
r = \frac{A}{s}
\]

where \( A \) is the area of the triangle, and \( s \) is the semi-perimeter.

Let's calculate the semi-perimeter \( s \):

\[
s = \frac{9 + 12 + 15}{2} = \frac{36}{2} = 18 \, \text{m}
\]

Now, calculate the area \( A \) of the right triangle. Since the legs are perpendicular, the area is:

\[
A = \frac{1}{2} \times \text{base} \times \text{height} = \frac{1}{2} \times 9 \times 12 = \frac{1}{2} \times 108 = 54 \, \text{m}^2
\]

Now, substitute the values into the inradius formula:

\[
r = \frac{A}{s} = \frac{54}{18} = 3 \, \text{m}
\]

Alternatively, for a right triangle, there is a specific formula for the inradius:

\[
r = \frac{a + b - c}{2}
\]

where \( a \) and \( b \) are the legs, and \( c \) is the hypotenuse. Plugging in the values:

\[
r = \frac{9 + 12 - 15}{2} = \frac{6}{2} = 3 \, \text{m}
\]

Both methods confirm the same result. Therefore, the radius of the inscribed circle is:

\[
\boxed{3 \, \text{m}}
\]

