# Prompt #20: A robot occupies an interval 0.6 m wide centered at +2 m. An obstacle is a 0.4 m interval centered at +3 m. Do they collide?

## OpenAI o3
No.

Half-widths  
• Robot: 0.6 m ÷ 2 = 0.30 m  
• Obstacle: 0.4 m ÷ 2 = 0.20 m  

Sum of the half-widths = 0.30 m + 0.20 m = 0.50 m

Center separation = |3 m − 2 m| = 1.0 m

Because the center separation (1.0 m) is greater than the sum of the half-widths (0.50 m), the two intervals do not overlap. Therefore the robot and the obstacle do not collide; there is a 0.5 m gap between them.

## Claude 4 Sonnet
To determine if they collide, I need to find if the two intervals overlap.

Robot interval:
- Center: +2 m
- Width: 0.6 m
- Left edge: 2 - 0.6/2 = 2 - 0.3 = 1.7 m
- Right edge: 2 + 0.6/2 = 2 + 0.3 = 2.3 m
- Robot occupies: [1.7 m, 2.3 m]

Obstacle interval:
- Center: +3 m  
- Width: 0.4 m
- Left edge: 3 - 0.4/2 = 3 - 0.2 = 2.8 m
- Right edge: 3 + 0.4/2 = 3 + 0.2 = 3.2 m
- Obstacle occupies: [2.8 m, 3.2 m]

Checking for overlap:
- Robot's rightmost point: 2.3 m
- Obstacle's leftmost point: 2.8 m

Since 2.3 m < 2.8 m, there is a gap of 0.5 m between them.

**No, they do not collide.**

## Gemini Pro
Let's determine the intervals occupied by the robot and the obstacle.

1.  **Robot:**
    *   Center: +2 m
    *   Width: 0.6 m
    *   Half-width: 0.6 m / 2 = 0.3 m
    *   Left edge of robot: 2 m - 0.3 m = 1.7 m
    *   Right edge of robot: 2 m + 0.3 m = 2.3 m
    *   **Robot occupies the interval [1.7 m, 2.3 m]**

2.  **Obstacle:**
    *   Center: +3 m
    *   Width: 0.4 m
    *   Half-width: 0.4 m / 2 = 0.2 m
    *   Left edge of obstacle: 3 m - 0.2 m = 2.8 m
    *   Right edge of obstacle: 3 m + 0.2 m = 3.2 m
    *   **Obstacle occupies the interval [2.8 m, 3.2 m]**

Now, let's compare the intervals:
*   Robot's interval: [1.7, 2.3]
*   Obstacle's interval: [2.8, 3.2]

The rightmost point of the robot is 2.3 m.
The leftmost point of the obstacle is 2.8 m.

Since 2.3 m < 2.8 m, the robot's interval ends before the obstacle's interval begins. There is no overlap.

**Conclusion:** No, they do not collide.

## DeepSeek Reasoner
The robot occupies an interval from \(2 - 0.3 = 1.7\) m to \(2 + 0.3 = 2.3\) m, since it is centered at +2 m with a width of 0.6 m (half-width is 0.3 m).

The obstacle occupies an interval from \(3 - 0.2 = 2.8\) m to \(3 + 0.2 = 3.2\) m, since it is centered at +3 m with a width of 0.4 m (half-width is 0.2 m).

The robot's interval ends at 2.3 m, and the obstacle's interval begins at 2.8 m. Since 2.3 m is less than 2.8 m, there is no overlap between the intervals. Specifically, there is a gap of \(2.8 - 2.3 = 0.5\) m between them.

Therefore, the robot and the obstacle do not collide.

**Key points:**
- Collision occurs only if the intervals overlap, meaning there is some point common to both intervals.
- Here, the robot is entirely to the left of the obstacle with a clear separation.

Alternatively, the distance between the centers is \(3 - 2 = 1\) m. The sum of the half-widths is \(0.3 + 0.2 = 0.5\) m. Since the distance between centers (1 m) is greater than the sum of the half-widths (0.5 m), there is no collision.

## Grok 3
To determine if the robot and the obstacle collide, we need to check if their intervals overlap. Let's define the intervals based on the given information.

- **Robot's interval**: The robot occupies a 0.6 m wide interval centered at +2 m.  
  Half of the width is 0.6 / 2 = 0.3 m.  
  Therefore, the robot's interval extends from:  
  2 - 0.3 = 1.7 m to 2 + 0.3 = 2.3 m.  
  So, the robot's interval is [1.7 m, 2.3 m].

- **Obstacle's interval**: The obstacle occupies a 0.4 m wide interval centered at +3 m.  
  Half of the width is 0.4 / 2 = 0.2 m.  
  Therefore, the obstacle's interval extends from:  
  3 - 0.2 = 2.8 m to 3 + 0.2 = 3.2 m.  
  So, the obstacle's interval is [2.8 m, 3.2 m].

- **Collision check**: Two intervals collide (overlap) if one interval's start is less than the other interval's end and vice versa. Mathematically, intervals [a, b] and [c, d] overlap if a < d and c < b.  
  Here, for the robot [1.7, 2.3] and obstacle [2.8, 3.2]:  
  - Is 1.7 < 3.2? Yes.  
  - Is 2.8 < 2.3? No.  

Since both conditions must be true for overlap to occur, and the second condition is false, the intervals do not overlap.

Alternatively, we can observe that the robot's interval ends at 2.3 m, and the obstacle's interval starts at 2.8 m. Since 2.3 m is less than 2.8 m, there is a gap between the two intervals (specifically, 2.8 - 2.3 = 0.5 m), confirming they do not touch or overlap.

**Conclusion**: The robot and the obstacle do not collide.

