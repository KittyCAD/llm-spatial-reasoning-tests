# llm-spatial-reasoning-tests

A set of prompts for testing LLMs spatial reasoning. The LLMs are wordcels and we need a shape rotator.

**TLDR;** the LLMs are great at math in N-dimensions (we tested 1, 2, 3, 4, & 5). **BUT** when it stops being raw math and starts getting physical, they start to break down.

## Models tested

- o3 🦖
- Claude Opus 4 🎶
- Gemini 2.5 Pro Preview ♊️
- DeepSeek R1 🐋
- Grok 3 🤖

## Prompts tried

| # | **Skill probed** | **Example prompt** | **Why it’s tricky** | **Result file** | **Who was right** |
|----|-----------------------------------|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|------------------------------------------------------------------------------------------------|-----------------|---|
| 1 | Mental rotation | Imagine a block letter ‘F’. Rotate it 90 ° clockwise in the plane, then spin it 180 ° about the vertical axis (like turning a page). Draw the result in ASCII. | Two independent rotations plus left–right reversal. | [prompt 01](results/prompt_01.md) | 🐋|
| 2 | 3-D coordinate transforms | A point is at (2, 1, −3) in camera space. The camera is at (0, 0, 0) looking down −Z with a 90 ° FOV. What pixel row/column (in a 1920 × 1080 sensor) does the point project to? | Chains perspective math **and** pixel indexing. | [prompt 02](results/prompt_02.md) | not evaluated yet |
| 3 | Cross-sections | Slice a cube with a plane that passes through the mid-points of three mutually adjacent edges. What shape is the cross-section? | Planes and cutting. | [prompt 03](results/prompt_03.md) | 🦖♊️🐋🤖|
| 4 | Folding / paper-punch | Fold a square sheet along its vertical center line. Punch a hole 1 cm from the top-left corner of the folded sheet. Unfold it—where are the holes relative to the original corners? | Combines mirroring with distance preservation. | [prompt 04](results/prompt_04.md) | 🐋 (the closest answer) |
| 5 | Occlusion & visibility | Camera at (0,0,0) looks toward +Z. Wall #1 is 4 m × 4 m, 10 m away. Wall #2 is 2 m × 2 m, 5 m away, centered in front of Wall #1. What percent of Wall #1’s area is visible? | Needs solid-angle reasoning and area ratios. | [prompt 05](results/prompt_05.md) | 🦖♊️🐋🤖|
| 6 | Path-planning / collision | A circular robot with radius 0.4 m must move from (0,0) to (6,0). A 1 m-wide corridor has centerline (0,0)→(3,2)→(6,0). Can the robot stay inside the corridor the whole way? | Must buffer corridor by robot radius and check clearance. | [prompt 06](results/prompt_06.md) | 🦖♊️🐋🤖 |
| 7 | Reference-frame concatenation | Gripper frame G is rotated 30 ° about X relative to robot frame R. Tool frame T is rotated 45 ° about Y relative to G. Give the 3 × 3 rotation matrix from T to R. | Classic robotics transform stack—easy to typo. | [prompt 07](results/prompt_07.md) | 🦖🎶♊️🐋🤖 |
| 8 | Net identification | Here’s an unfolded net of six squares in a T-shape. When folded, which squares share an edge with the center square? | Requires mentally tucking flaps. | [prompt 08](results/prompt_08.md) | 🦖🎶♊️🐋 (diff interpretations) |
| 9 | Symmetry & chirality | Is the right-hand rule still valid if you swap the Y and Z axes but leave X alone? Why / why not? | Tests awareness of handedness parity. | [prompt 09](results/prompt_09.md) | 🦖🎶♊️🐋🤖 |
| 10 | Mixed-unit Pythagorean | Object A is 15 cm in X and 8 in in Y. Object B is 120 mm in X and 0.1 m in Y. Which is closer? | Unit juggling + Pythagoras. | [prompt 10](results/prompt_10.md) | 🦖🎶♊️🐋🤖 |
| 11 | Topological reasoning | A torus and a coffee mug are homeomorphic. Describe a continuous deformation turning the mug into the torus without tearing or gluing. | Separates parroting from genuine genus-1 insight. | [prompt 11](results/prompt_11.md) | not evaluated yet |
| 12 | Mirror-image deduction | You stand at (0,0,0) facing +Y. A mirror is the plane X = 3. Where will your reflection appear? | Needs plane-reflection formula. | [prompt 12](results/prompt_12.md) | not evaluated yet |
| 13 | Signed translation (1-D) | A point starts at +7 cm, slides −18 cm. Where does it end up, and is it left or right of the origin? | Positive/negative offsets & verbal direction. | [prompt 13](results/prompt_13.md) | 🦖🎶♊️🐋🤖 |
| 14 | Interval overlap & length | Segment A spans [−3 m, +9 m]; segment B spans [+4 m, +15 m]. How many meters (and what percent of A) do they overlap? | Requires max-min overlap logic. | [prompt 14](results/prompt_14.md) | 🦖🎶♊️🐋🤖 |
| 15 | 1-D mirror reflection | Reflect x = −12 across x = +5. Where is the image? | Uses formula x′ = 2c − x. | [prompt 15](results/prompt_15.md) | 🦖🎶♊️🐋🤖 |
| 16 | Relative motion / chase | Car X at 0 km moves +90 km/h. Car Y at +300 km moves −60 km/h. When and where do they meet? | One-dimensional pursuit with signed velocities. | [prompt 16](results/prompt_16.md) | 🦖🎶♊️🐋🤖 |
| 17 | Folding a strip | Fold a 20 cm strip at 14 cm so ends align. After folding, how far apart are the original 0 cm and 20 cm marks? | Distance halves only where folded—easy to miscount. | [prompt 17](results/prompt_17.md) | not evaluated yet |
| 18 | Unit conversion (1-D) | Point A is 250 mm from origin. Point B is 1 ft right of A. How far is B from origin in cm? | mm → cm → ft conversions chain. | [prompt 18](results/prompt_18.md) | 🦖🎶♊️🐋🤖 |
| 19 | Ordering after shifts | Points P = −8, Q = 4, R = 10; shift each +13 and list them left→right with new coordinates. | Must re-sort after translation. | [prompt 19](results/prompt_19.md) | 🦖🎶♊️🐋🤖 |
| 20 | 1-D collision spacing | Robot interval 0.6 m wide centered at +2 m; obstacle interval 0.4 m centered at +3 m. Do they collide? | Expand to half-widths & check interval overlap. | [prompt 20](results/prompt_20.md) | 🦖🎶♊️🐋🤖 |
| 21 | Boolean interval logic | Is (−2 < x ≤ 5) **and** (x > 3) equivalent to 3 < x ≤ 5? Explain. | Symbolic simplification on a line. | [prompt 21](results/prompt_21.md) | not evaluated yet |
| 22 | Discrete stepping & parity | Start at −7, step +5 until past +20. List every landing spot and mark even/odd. | Loop logic plus parity—all 1-D. | [prompt 22](results/prompt_22.md) | 🦖🎶♊️🐋🤖 |
| 23 | Composite perimeter (2-D) | Two 4 cm × 4 cm squares share one corner; interiors don't overlap. Draw the outline and state the total perimeter. | Must visualize L-shape and avoid double-counting edges. | [prompt 23](results/prompt_23.md) | 🦖🎶♊️🐋 |
| 24 | In-circle radius (2-D) | Right-triangle legs 9 m and 12 m: find the inscribed circle’s radius. | Uses r = (a + b − c)/2 or area / semiperimeter. | [prompt 24](results/prompt_24.md) | 🦖🎶♊️🐋🤖 |
| 25 | Polygon path & displacement | Robot walks 10 m E, 10 m N, 10 m W, 10 m S. What is net displacement and what closed shape is traced? | Zero displacement; traces a square—checks closure. | [prompt 25](results/prompt_25.md) | 🦖🎶♊️🐋🤖 |
| 26 | Translation & overlap (2-D) | Rectangle R (−3,−1)→(7,6) shifted (−4,−2) → S. Give S corners and the overlap area of R and S. | Combine vector shift with rectangle intersection math. | [prompt 26](results/prompt_26.md) | 🦖🎶♊️🐋🤖 |
| 27 | Circumscribed circle (2-D) | Regular hexagon side 5 cm is circumscribed by a circle. Find the circle’s radius and fraction of its area outside the hexagon. | Needs r = s and area ratio πr² to 6·(√3/4)s². | [prompt 27](results/prompt_27.md) | 🦖🎶♊️🐋🤖 |
| 28 | Fold-and-punch (2-D) | Square 16 cm folded on diagonal; punch 2 cm from fold & 5 cm from edge. After unfolding, mark all hole locations. | Mirror across the diagonal—yields two symmetric holes. | [prompt 28](results/prompt_28.md) | 🦖🎶♊️🐋🤖 |
| 29 | Rotation + reflection (2-D) | Rotate (2, 5) 120° CCW about origin, then reflect across x-axis. Where is the point now? | Chains rotation matrix with sign flip. | [prompt 29](results/prompt_29.md) | 🦖🎶♊️🐋🤖 |
| 30 | Offset frame area (2-D) | 3 cm-wide path inside a 20 × 12 cm rectangle (like an inner frame). Compute path area and remaining central area. | Outer minus inner rectangle; adjust both dimensions. | [prompt 30](results/prompt_30.md) | 🦖🎶♊️🐋🤖 |
| 31 | Circle-circle intersection | Two discs radius 4 cm, centers 6 cm apart. Find the area of their overlap (lens) to nearest 0.1 cm². | Uses segment-area formula with arccos; numeric heavy. | [prompt 31](results/prompt_31.md) | 🦖♊️🐋🤖 |
| 32 | Centroid & median (2-D) | Triangle (0,0), (8,0), (3,6): compute centroid and equation of median from (3,6). | Combines centroid averaging and point-slope line. | [prompt 32](results/prompt_32.md) | 🦖🎶♊️🐋🤖 |
| 33 | 5-D Euclidean norm | I have a 5-dimensional point (2, −5, 10, 3, 2). What is the magnitude of this point? | Extends √(x²+y²+z²+i²+j²); easy to drop one term. | [prompt 33](results/prompt_33.md) | 🦖🎶♊️🐋🤖 |
| 34 | 4-D single-axis rotation | A 4-D point (2, −5, 10, 3). Rotate it 90 ° about the z-axis. | Must treat rotation in x-y plane while z & i components behave correctly. | [prompt 34](results/prompt_34.md) | 🦖🎶♊️🐋🤖|
| 35 | 4-D Euclidean distance | In ℝ⁴ the points A (1, −2, 4, 0) and B (−3, 6, 1, 5). Find the exact distance. | Adds a fourth squared-difference term. | [prompt 35](results/prompt_35.md) | 🦖🎶♊️🐋🤖|
| 36 | Tesseract cross-section | 4-cube edge 2 sliced by x + y + z + w = 1. Describe the 3-D polyhedron & its volume. | Visualising a hyper-plane cut; volume of 3-D slice. | [prompt 36](results/prompt_36.md) | not evaluated yet |
| 37 | 4→3 projection & scaling | Project (3, −1, 2, 5) onto w = 1, then scale by ½. Give final coords. | Orthographic drop of w, then uniform scale. | [prompt 37](results/prompt_37.md) | 🦖🎶♊️🐋🤖|
| 38 | Compound 4-D rotation | Give the 4×4 matrix for 15° in x-y then 30° in z-w (RH rule each). | Two independent plane rotations; sign/order pitfalls. | [prompt 38](results/prompt_38.md) | 🦖🎶♊️🐋🤖|
| 39 | Hyper-volume of 4-sphere | Hyper-volume of radius-7 4-sphere, in terms of π. | Uses V₄ = ½ π² r⁴; many forget the factor. | [prompt 39](results/prompt_39.md) | 🦖🎶♊️🐋🤖 |
| 40 | 4-D constant-velocity motion | Start (0,0,0,0), v = (1, −2, 3, 4) u/s. Where after √3 s? | Vector-time product with radicals. | [prompt 40](results/prompt_40.md) | 🦖🎶♊️🐋🤖 |
| 41 | Minkowski interval classification | Events P(5,3,4,0) & Q(10,9,7,2) in signature (−+++). Compute interval & classify. | Correct sign convention; decide time/space/light-like. | [prompt 41](results/prompt_41.md) | 🦖🎶♊️🐋🤖 |
| 42 | Oriented 4-volume / determinant | Vectors a=(1,0,0,1), b=(0,1,0,1), c=(0,0,1,1), d=(1,1,1,1): find oriented 4-volume & handedness. | 4-D determinant; sign gives orientation. | [prompt 42](results/prompt_42.md) | |
| 43 | 2-D rotation + reflection | L-tetromino at (0,0)(1,0)(2,0)(2,1); rotate 270° CCW about origin, then reflect across y = x. List new coords ascending by x. | Combines large rotation with diagonal mirror—easy to drop a block. | [prompt 43](results/prompt_43.md) |
| 44 | Cone cross-section scaling | Right cone (h 8 cm, r 3 cm) cut 3 cm below apex, plane ∥ base → What shape & area? | Needs similar-triangles scaling and area formula. | [prompt 44](results/prompt_44.md) |
| 45 | Out-of-plane flip (ASCII) | Block letter “R”, flip 180° about horizontal in-plane axis. ASCII-draw result. | Forces mental 3-D flip; many confuse with vertical mirror. | [prompt 45](results/prompt_45.md) |
| 46 | Arbitrary-axis 3-D rotation | Rotate P = (1,2,−4) 120° about ⟨1,1,1⟩ axis. Give exact coords. | Requires Rodrigues/homogeneous rotation math. | [prompt 46](results/prompt_46.md) |
| 47 | Cube slice → polygon vertices | Unit cube, plane through (0,½,½), (½,0,½), (½,½,0). List resulting polygon’s vertices cyclically. | Identify 3-D intersection edges; yields regular hexagon cross-section. | [prompt 47](results/prompt_47.md) |
| 48 | 4 → 3 projection + 2-D rotation | Project (7,−3,5,2) to w = 0; rotate 90° about z. Give final 3-D point. | Mixes hyper-projection with classic xy-plane spin. | [prompt 48](results/prompt_48.md) |
| 49 | Rotational symmetry reasoning | Regular dodecagon, keep every 2nd vertex. What polygon results? Explain. | Needs n/2 rule → regular hexagon proof. | [prompt 49](results/prompt_49.md) |
| 50 | Path buffer / Minkowski sum | Robot radius 0.5 m along polyline (0,0)→(2,1)→(4,1). Is 1 m-wide corridor enough? | Buffer polyline by radius and check narrowest segment. | [prompt 50](results/prompt_50.md) |
| 51 | 3-D rotation of extrusion | Equilateral Δ side 6 cm extruded 10 cm (+z). Rotate prism 90° about y. New coords of (0,0,0). | Apply y-axis rotation to 3-D vertex; mixes extrusion + rotation. | [prompt 51](results/prompt_51.md) |
| 52 | Spatial algorithm design | Two random L-tetrominoes on 4×4 grids. Outline algorithm (no reflection) to test rotational equivalence. | Meta-spatial reasoning, not brute enumeration. | [prompt 52](results/prompt_52.md) |

### Methodology behind the prompts

- **1-12** → 2-D/3-D space
- **13–22** → 1-D & 2-D fundamentals
- **23–32** → Advanced 2-D set
- **33–42** → 4-D/5-D hyper-spatial
- **43–52** → “Shape-rotator” mental-rotation specials
