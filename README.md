# llm-spatial-reasoning-tests

A set of prompts for testing LLMs spatial reasoning.

**Models included:**

- o3
- Claude Opus 4
- Gemini 2.5 Pro Preview
- DeepSeek R1
- Grok 3

| # | **Skill probed** | **Example prompt** | **Why it’s tricky** | **Result file** |
|----|-----------------------------------|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|------------------------------------------------------------------------------------------------|-----------------|
| 1 | Mental rotation | Imagine a block letter ‘F’. Rotate it 90 ° clockwise in the plane, then spin it 180 ° about the vertical axis (like turning a page). Draw the result in ASCII. | Two independent rotations plus left–right reversal. | [prompt 01](results/prompt_01.md) |
| 2 | 3-D coordinate transforms | A point is at (2, 1, −3) in camera space. The camera is at (0, 0, 0) looking down −Z with a 90 ° FOV. What pixel row/column (in a 1920 × 1080 sensor) does the point project to? | Chains perspective math **and** pixel indexing. | [prompt 02](results/prompt_02.md) |
| 3 | Cross-sections | Slice a cube with a plane that passes through the mid-points of three mutually adjacent edges. What shape is the cross-section? | Many LLMs guess triangle; it’s a **regular hexagon**. | [prompt 03](results/prompt_03.md) |
| 4 | Folding / paper-punch | Fold a square sheet along its vertical center line. Punch a hole 1 cm from the top-left corner of the folded sheet. Unfold it—where are the holes relative to the original corners? | Combines mirroring with distance preservation. | [prompt 04](results/prompt_04.md) |
| 5 | Occlusion & visibility | Camera at (0,0,0) looks toward +Z. Wall #1 is a 4 m × 4 m square 10 m away. Wall #2 is a 2 m × 2 m square 5 m away, centered in front of Wall #1. What percent of Wall #1’s area is visible? | Needs solid-angle reasoning and area ratios. | [prompt 05](results/prompt_05.md) |
| 6 | Path-planning / collision | A circular robot with radius 0.4 m must move from (0,0) to (6,0). There’s a 1 m-wide corridor whose centerline is the polyline (0,0)→(3,2)→(6,0). Can the robot stay inside the corridor the whole way? | Must buffer corridor by robot radius and check clearance. | [prompt 06](results/prompt_06.md) |
| 7 | Reference-frame concatenation | Gripper frame G is rotated 30 ° about X relative to robot frame R. Tool frame T is rotated 45 ° about Y relative to G. Give the 3 × 3 rotation matrix from T to R. | Classic robotics transform stack—easy to typo. | [prompt 07](results/prompt_07.md) |
| 8 | Net identification | Here’s an unfolded net of six squares in a T-shape. When folded, which squares share an edge with the center square? | Requires mentally tucking flaps. | [prompt 08](results/prompt_08.md) |
| 9 | Symmetry & chirality | Is the right-hand rule still valid if you swap the Y and Z axes but leave X alone? Why / why not? | Tests awareness of handedness parity. | [prompt 09](results/prompt_09.md) |
| 10 | Mixed-unit Pythagorean | Object A is 15 cm away in X and 8 in away in Y. Object B is 120 mm away in X and 0.1 m in Y. Which is closer? | Unit juggling + Pythagoras. | [prompt 10](results/prompt_10.md) |
| 11 | Topological reasoning | A torus and a coffee mug are homeomorphic. Name a continuous deformation that turns the mug into the torus without tearing or gluing. | Separates parroting from genuine genus-1 insight. | [prompt 11](results/prompt_11.md) |
| 12 | Mirror-image deduction | You stand at (0,0,0) facing +Y. A mirror is the plane X = 3. Where will your reflection appear? | Needs plane-reflection formula. | [prompt 12](results/prompt_12.md) |
| 13 | Signed translation (1-D) | A point starts at +7 cm. It slides −18 cm along the line. Where does it end up, and is it left or right of the origin? | Positive/negative offsets & verbal direction. | [prompt 13](results/prompt_13.md) |
| 14 | Interval overlap & length | Segment A spans [−3 m, +9 m]. Segment B spans [+4 m, +15 m]. What portion (in meters and as a percent of A) do they overlap? | Requires max(min) overlap logic. | [prompt 14](results/prompt_14.md) |
| 15 | 1-D mirror reflection | Reflect the point x = −12 across the line x = +5. Where is the image? | Uses formula x′ = 2c − x. | [prompt 15](results/prompt_15.md) |
| 16 | Relative motion / chase | Car X is at 0 km heading + direction at 90 km/h. Car Y is at +300 km heading − direction at 60 km/h. When and where do they meet? | One-dimensional pursuit with signed velocities. | [prompt 16](results/prompt_16.md) |
| 17 | Folding a strip | Fold a 20 cm paper strip at the 14 cm mark so the ends align. After the fold, how far apart are the original 0 cm and 20 cm points? | Distance halves only where folded—easy to mis-count. | [prompt 17](results/prompt_17.md) |
| 18 | Unit conversion (1-D) | Point A is 250 mm from the origin. Point B is 1 ft to the right of A. How far is B from the origin in cm? | mm → cm → ft conversions chain. | [prompt 18](results/prompt_18.md) |
| 19 | Ordering after shifts | Points P = −8, Q = 4, R = 10; shift +13. Sort them left→right with their new coordinates. | Must re-sort after translation. | [prompt 19](results/prompt_19.md) |
| 20 | 1-D collision spacing | A robot occupies an interval 0.6 m wide centered at +2 m. An obstacle is a 0.4 m interval centered at +3 m. Do they collide? | Expand to half-widths & check interval overlap. | [prompt 20](results/prompt_20.md) |
| 21 | Boolean interval logic | Is the inequality −2 < x ≤ 5 **and** x > 3 equivalent to 3 < x ≤ 5? Explain. | Symbolic simplification on a line. | [prompt 21](results/prompt_21.md) |
| 22 | Discrete stepping & parity | Starting at −7, move right in steps of 5 until you pass +20. List every landing spot and state whether each is even or odd. | Loop logic plus parity—all 1-D. | [prompt 22](results/prompt_22.md) |
