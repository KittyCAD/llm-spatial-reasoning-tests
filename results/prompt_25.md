# Prompt #25: Starting at (0,0), a robot walks 10 m east, then 10 m north, then 10 m west, then 10 m south. What is the net displacement, and what closed-shape was traced?

## OpenAI o3
Path followed (with coordinates in metres)

1. Start: (0, 0)  
   Walk 10 m east → (10, 0)

2. Walk 10 m north → (10, 10)

3. Walk 10 m west → (0, 10)

4. Walk 10 m south → (0, 0) (back to the start)

Net displacement  
Final position – initial position = (0, 0) – (0, 0) = (0, 0)  
Magnitude = 0 m.  
So the robot’s net displacement is zero.

Shape traced  
The four straight segments of equal length meeting at right angles form a closed square of side 10 m.

## Claude 4 Sonnet
I need to track the robot's movement step by step and determine its final position.

Starting position: (0, 0)

Step 1: 10 m east
- New position: (10, 0)

Step 2: 10 m north
- New position: (10, 10)

Step 3: 10 m west
- New position: (0, 10)

Step 4: 10 m south
- New position: (0, 0)

The robot returns to its starting position at (0, 0).

**Net displacement**: 0 meters
- Displacement is the straight-line distance from start to finish
- Since the robot ends where it started, the displacement is zero

**Closed shape traced**: Square
- The robot traced out a square with sides of length 10 m
- The vertices of the square are at: (0, 0), (10, 0), (10, 10), and (0, 10)
- The robot walked along the perimeter in a clockwise direction

## Gemini Pro
Let's break down the robot's movement:

1.  **Starts at (0,0)**
2.  **10 m east:** Moves from (0,0) to (10,0)
3.  **10 m north:** Moves from (10,0) to (10,10)
4.  **10 m west:** Moves from (10,10) to (0,10)
5.  **10 m south:** Moves from (0,10) to (0,0)

**Net Displacement:**
Displacement is the straight-line distance and direction from the starting point to the ending point.
*   Starting point: (0,0)
*   Ending point: (0,0)
Since the robot returns to its starting point, the **net displacement is 0 meters**.

**Closed Shape Traced:**
The robot traced out a path with four equal sides (each 10 m) and four right-angle turns (East to North, North to West, West to South, South to East/Start). This shape is a **square**.

## DeepSeek Reasoner
The robot starts at (0,0).  
- Moving 10 m east takes it to (10, 0).  
- Moving 10 m north from there takes it to (10, 10).  
- Moving 10 m west from there takes it to (0, 10).  
- Moving 10 m south from there takes it back to (0, 0).  

The net displacement is the straight-line distance from the starting point to the ending point. Since the robot ends at the starting point (0, 0), the net displacement is 0 m.  

The path traced by the robot is from (0,0) to (10,0) to (10,10) to (0,10) and back to (0,0). This path forms a square with sides of 10 m, oriented such that the sides are parallel to the cardinal directions (east-west and north-south).  

Thus, the net displacement is 0 m, and the closed shape traced is a square.

\boxed{0} \text{ m} \\
\boxed{\text{square}}

## Grok 3
Let's analyze the robot's movement step by step to determine the net displacement and the shape traced.

1. **Initial Position**: The robot starts at the origin, (0,0).
2. **Step 1**: Walks 10 m east. This moves the robot along the positive x-axis to (10,0).
3. **Step 2**: Walks 10 m north. This moves the robot along the positive y-axis to (10,10).
4. **Step 3**: Walks 10 m west. This moves the robot along the negative x-axis to (0,10).
5. **Step 4**: Walks 10 m south. This moves the robot along the negative y-axis back to (0,0).

**Net Displacement**: Displacement is the straight-line distance from the starting point to the ending point. Since the robot starts at (0,0) and ends at (0,0), the net displacement is 0 meters. (Note: Displacement is a vector quantity, but since the starting and ending points are the same, the magnitude is 0.)

**Shape Traced**: The path taken by the robot forms a closed shape. Let's visualize the path:
- From (0,0) to (10,0) — east.
- From (10,0) to (10,10) — north.
- From (10,10) to (0,10) — west.
- From (0,10) to (0,0) — south.

This path connects four points: (0,0), (10,0), (10,10), and (0,10), and returns to the starting point. The shape traced is a square with each side of length 10 meters.

**Final Answer**:
- Net displacement: 0 meters.
- Closed shape traced: Square.

