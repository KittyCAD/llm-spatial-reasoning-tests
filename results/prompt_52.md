# Prompt #52: Two identical L-tetrominoes are placed randomly on separate 4 × 4 grids. Provide an algorithmic recipe (in prose) that determines whether one can be rotated (no reflection) to match the other.

## OpenAI o3  (19 secs)
Below is a straight-forward, “coding–ready’’ outline that decides, for two
independent 4 × 4 boards, whether the L-tetromino shown on board A can be
brought to coincide with the one shown on board B using rotations only
(0°, 90°, 180°, 270°), never a mirror reflection.

────────────────────────────────────────────────────────────────────────
1.  Pick a way to read a board
────────────────────────────────────────────────────────────────────────
    • Represent every board as a 4 × 4 Boolean array  
      (true = square is covered by the L, false = empty).

    • While reading the board store the coordinates of the four covered
      squares in any convenient container; a list or a set is fine:
          S  = {(r1,c1), … ,(r4,c4)}  with  0 ≤ r,c ≤ 3.

────────────────────────────────────────────────────────────────────────
2.  Normalise a coordinate set
────────────────────────────────────────────────────────────────────────
    Normalisation removes the influence of translation.  
    Given a set S

        let   rmin = min r over S
              cmin = min c over S
        return { (r – rmin , c – cmin)  |  (r,c) ∈ S }.

    After this step the “top-left’’ occupied cell is always (0,0).

────────────────────────────────────────────────────────────────────────
3.  How to rotate one coordinate
────────────────────────────────────────────────────────────────────────
    The four rotations that keep the squares on an (unbounded) grid are

        0°  :  (r , c)      → ( r ,  c)                (identity)
        90° :  (r , c)      → ( c , –r)                (about origin)
        180°:  (r , c)      → (–r , –c)
        270°:  (r , c)      → (–c ,  r).

    We will apply the chosen formula to every member of the set.  
    (Negative coordinates are fine; they disappear after renormalising.)

────────────────────────────────────────────────────────────────────────
4.  The comparison algorithm – high level
────────────────────────────────────────────────────────────────────────
    INPUT  : boardA, boardB.
    OUTPUT : TRUE  if A can be rotated to match B,  
             FALSE otherwise.

    Step 1   Take the four coordinates of board B,
             normalise them once:   NB = normalise(B).

    Step 2   Take the four coordinates of board A: call this set SA.

    Step 3   For each of the four rotation angles θ ∈ {0°,90°,180°,270°}

                 Rθ = { rotateθ(p)  |  p ∈ SA }     // rotate every cell
                 Nθ = normalise(Rθ)                  // kill translation

                 if  Nθ == NB      // set equality, order irrelevant
                     return TRUE

    Step 4   If the loop finishes without a match, return FALSE.

────────────────────────────────────────────────────────────────────────
5.  Correctness sketch
────────────────────────────────────────────────────────────────────────
    • Renormalisation makes all translations irrelevant.
    • The four bodies Nθ produced in the loop are exactly the four
      translational classes reachable from board A by pure rotation.
      If none of them equals the (translation-free) class NB of board B
      the two L’s are not rotationally congruent inside the 4 × 4 board.
    • Therefore the procedure answers TRUE   ⇔   a rotation exists.

────────────────────────────────────────────────────────────────────────
6.  Runtime remarks
────────────────────────────────────────────────────────────────────────
    Each board has only 4 occupied cells, and the algorithm performs a
    constant amount of work on constant-sized sets:  
        time  ∈ Θ(1),   space ∈ Θ(1).

That is the entire “recipe’’—easy to code in any language, and easy to
verify by hand for small test cases.

## Claude 4 Sonnet  (14 secs)
Here's an algorithmic recipe to determine if two L-tetrominoes on separate 4×4 grids can be matched through rotation:

**Step 1: Extract the L-tetromino coordinates**
- For each grid, scan through all cells to find the four cells that contain the L-tetromino
- Store these four coordinates as a set of (row, column) pairs for each tetromino

**Step 2: Normalize each tetromino to a canonical position**
- For each tetromino:
  - Find the minimum row and minimum column values among its four cells
  - Translate the tetromino by subtracting these minimum values from all coordinates
  - This moves the tetromino to touch the top and left edges of an imaginary bounding box

**Step 3: Generate all four rotational variants of the first tetromino**
- Starting with the normalized first tetromino's coordinates:
  - **0° rotation**: Keep coordinates as-is
  - **90° rotation**: Transform each (r,c) to (c, -r), then normalize again
  - **180° rotation**: Transform each (r,c) to (-r, -c), then normalize again  
  - **270° rotation**: Transform each (r,c) to (-c, r), then normalize again

**Step 4: Compare with the second tetromino**
- Sort the coordinates of the normalized second tetromino (for consistent comparison)
- For each of the four rotational variants from Step 3:
  - Sort the variant's coordinates
  - Check if the sorted coordinates exactly match the sorted coordinates of the second tetromino
  - If any match is found, the tetrominoes can be rotated to match each other

**Step 5: Return the result**
- If a match was found in Step 4, return "Yes, they can be matched by rotation"
- Otherwise, return "No, they cannot be matched by rotation alone"

This algorithm works because L-tetrominoes have a unique shape that can exist in exactly 4 rotational orientations, and by normalizing their positions, we can compare their shapes independent of their placement on the grid.

## Gemini Pro  (101 secs)
Here's an algorithmic recipe in prose to determine if one L-tetromino can be rotated (no reflection) to match another, given they are on separate 4x4 grids:

**Overall Goal:**
We want to compare Tetromino A with Tetromino B. Since Tetromino B can be rotated, we'll compare Tetromino A to all 4 possible rotational orientations of Tetromino B (0, 90, 180, and 270 degrees). To make these comparisons meaningful regardless of their positions on their respective grids, we'll convert each tetromino (or its rotated version) into a "canonical form."

**1. Representing a Tetromino:**
   - Each tetromino is initially defined by a list of four (row, column) coordinate pairs, indicating the cells it occupies on its 4x4 grid. Assume coordinates are 0-indexed (e.g., rows 0-3, columns 0-3).

**2. Procedure: Generating the Canonical Form of a Tetromino**
   This procedure takes a list of 4 coordinate pairs and returns a standardized representation. Let's call this `GetCanonicalForm`.
   a.  **Translate to Origin (Relative Coordinates):**
      i.  Find the minimum row value among the 4 cells of the tetromino. Subtract this minimum row value from the row coordinate of all 4 cells. (This effectively shifts the tetromino upwards so its top-most cell is in row 0 of a conceptual relative grid).
      ii. Find the minimum column value among these 4 (now row-translated) cells. Subtract this minimum column value from the column coordinate of all 4 cells. (This shifts the tetromino leftwards so its left-most cell is in column 0 of the relative grid, without altering its vertical alignment from the previous step).
   b.  **Sort for Uniqueness:**
      Sort the resulting 4 (row, column) coordinate pairs. A standard way to sort is lexicographically: sort primarily by the row value, and then by the column value for any ties.
   c.  **Output:** The sorted list of 4 translated coordinate pairs is the canonical form for that specific orientation of the tetromino.

**3. Procedure: Rotating a Tetromino 90 Degrees Clockwise**
   This procedure takes a list of 4 coordinate pairs representing a tetromino and returns a new list of coordinates for the tetromino after a 90-degree clockwise rotation. Let's call this `Rotate90Clockwise`.
   a.  **Choose a Pivot:** Select one cell from the input tetromino's coordinates to act as the pivot for rotation. A consistent choice is the first cell in its list of coordinates (e.g., if the coordinates are `[(r1,c1), (r2,c2), (r3,c3), (r4,c4)]`, use `(r1,c1)` as the pivot `(pr, pc)`).
   b.  **Rotate Each Cell:** For each cell `(r, c)` in the input tetromino:
      i.  Calculate its position relative to the pivot: `dr = r - pr`, `dc = c - pc`.
      ii. Apply a 90-degree clockwise rotation to these relative coordinates: the new relative row `new_dr` becomes `dc`, and the new relative column `new_dc` becomes `-dr`.
      iii. Translate the rotated relative cell back by adding the pivot coordinates: the new absolute cell coordinate is `(new_dr + pr, new_dc + pc)`.
   c.  **Output:** Collect these 4 new rotated cell coordinates into a list. This list represents the tetromino rotated by 90 degrees.

**4. Main Algorithm to Determine Rotational Match:**

   a.  **Input:** You are given `Coords_A` (coordinates for Tetromino A) and `Coords_B` (coordinates for Tetromino B).

   b.  **Standardize Tetromino A:**
      Calculate the canonical form of Tetromino A: `Canonical_A = GetCanonicalForm(Coords_A)`.

   c.  **Iterate Through Rotations of Tetromino B:**
      Let `Current_Coords_B = Coords_B`.
      We will check up to four orientations:
      i.  **0-degree rotation (Original B):**
          - Calculate `Canonical_B_current = GetCanonicalForm(Current_Coords_B)`.
          - If `Canonical_A` is identical to `Canonical_B_current`, the tetrominoes match. **Return `True`**.

      ii. **90-degree rotation:**
          - Rotate `Current_Coords_B` by 90 degrees: `Current_Coords_B = Rotate90Clockwise(Current_Coords_B)`.
          - Calculate `Canonical_B_current = GetCanonicalForm(Current_Coords_B)`.
          - If `Canonical_A` is identical to `Canonical_B_current`, they match. **Return `True`**.

      iii. **180-degree rotation:**
           - Rotate the (already 90-degree rotated) `Current_Coords_B` again: `Current_Coords_B = Rotate90Clockwise(Current_Coords_B)`.
           - Calculate `Canonical_B_current = GetCanonicalForm(Current_Coords_B)`.
           - If `Canonical_A` is identical to `Canonical_B_current`, they match. **Return `True`**.

      iv. **270-degree rotation:**
           - Rotate the (already 180-degree rotated) `Current_Coords_B` again: `Current_Coords_B = Rotate90Clockwise(Current_Coords_B)`.
           - Calculate `Canonical_B_current = GetCanonicalForm(Current_Coords_B)`.
           - If `Canonical_A` is identical to `Canonical_B_current`, they match. **Return `True`**.

   d.  **No Match Found:** If none of the above comparisons resulted in a match, then Tetromino A cannot be rotated to match Tetromino B. **Return `False`**.

**Explanation Notes:**
*   The "canonical form" ensures that two tetrominoes with the same shape and orientation will produce the exact same list of relative coordinates, regardless of where they are placed on their grids.
*   The rotation can result in coordinates that are temporarily negative or outside the 0-3 range of the grid. This is fine, as the `GetCanonicalForm` procedure will re-normalize them based on the shape's own extent.
*   The L-tetromino has 4 distinct rotational orientations (0, 90, 180, 270 degrees). Checking these four possibilities for Tetromino B against the single canonical form of Tetromino A is sufficient.

## DeepSeek Reasoner  (538 secs)
To determine whether one L-tetromino can be rotated (without reflection) to match the other on their respective 4×4 grids, follow this algorithmic recipe:

1. **Identify the cells for each tetromino**:  
   For both tetrominoes, list the grid coordinates of the four cells they occupy. Denote the first tetromino as A and the second as B.

2. **Iterate over possible centers of rotation for A**:  
   For each cell \( C \) in tetromino A, consider \( C \) as a potential center of rotation. There are up to four choices for \( C \) (one for each cell in A).

3. **Iterate over rotation angles**:  
   For each center \( C \), consider all four rotation angles: 0°, 90°, 180°, and 270° clockwise. For each angle, compute the new positions of all cells in A after rotation around \( C \) as follows:
   - Let \( C \) have coordinates \( (r_c, c_c) \), where \( r_c \) is the row and \( c_c \) is the column.
   - For each cell \( P \) in A with coordinates \( (r_p, c_p) \):
     - Compute the row difference \( dr = r_p - r_c \) and column difference \( dc = c_p - c_c \).
     - Apply the rotation to \( (dr, dc) \) to find the new differences:
       - For 0°: \( (dr_{\text{new}}, dc_{\text{new}}) = (dr, dc) \)
       - For 90°: \( (dr_{\text{new}}, dc_{\text{new}}) = (dc, -dr) \)
       - For 180°: \( (dr_{\text{new}}, dc_{\text{new}}) = (-dr, -dc) \)
       - For 270°: \( (dr_{\text{new}}, dc_{\text{new}}) = (-dc, dr) \)
     - Compute the new position of \( P \) as:
       \[
       (r_{\text{new}}, c_{\text{new}}) = (r_c + dr_{\text{new}}, c_c + dc_{\text{new}})
       \]
   - Collect the set of new coordinates for all cells in A after rotation.

4. **Check for match with B**:  
   Compare the set of new coordinates from the rotated A to the set of coordinates of B. If they are identical (i.e., the same four cells), then A can be rotated to match B using this center and angle. Return "yes" and terminate.

5. **Conclusion if no match found**:  
   If, after trying all centers \( C \) and all rotation angles, no rotated version of A matches B, return "no".

This algorithm works because it exhaustively checks all possible rotations of A around each of its cells (as potential centers) and verifies if any resulting position matches B exactly. Since rotations are limited to 90° increments and centers are chosen from A's cells, this covers all valid rotations on the grid without reflection.

**Example for clarity**:  
Suppose A occupies cells {(1,1), (1,2), (1,3), (2,1)} and B occupies cells {(1,2), (2,2), (3,2), (3,1)}.  
- Choosing C as (1,1) and rotating 90° clockwise:  
  - For P=(1,1): dr=0, dc=0 → (0,0) → new position (1+0, 1+0) = (1,1)  
  - For P=(1,2): dr=0, dc=1 → (1,0) for 90°? Wait, error in calculation:  
    Standard: for 90° CW, (dr, dc) → (dc, -dr)  
    P=(1,2): dr = 1-1=0, dc=2-1=1 → (1, -0) = (1,0) → new position (1 + 1, 1 + 0) = (2,1)? Correction:  
    New position: row = r_c + dc = 1 + 1 = 2, col = c_c - dr = 1 - 0 = 1 → (2,1)  
    Better to use formula directly:  
    new_row = r_c + dc, new_col = c_c - dr  
    So P=(1,2): dr=0, dc=1 → new_row = 1 + 1 = 2, new_col = 1 - 0 = 1 → (2,1)  
    P=(1,3): dr=0, dc=2 → new_row = 1 + 2 = 3, new_col = 1 - 0 = 1 → (3,1)  
    P=(2,1): dr=1, dc=0 → new_row = 1 + 0 = 1, new_col = 1 - 1 = 0 → invalid (col 0 not in 1-4)  
  Thus, rotation around (1,1) at 90° gives invalid cell, no match.  
- Other centers and angles must be checked. Eventually, choosing C as (1,3) and rotating 270° CW (or other combinations) might yield B, but in this case, no immediate match—algorithm would continue or return "no" if none found.  

This method ensures correctness for any valid placement on the 4×4 grid.

## Grok 3  (18 secs)
To determine whether two identical L-tetrominoes placed randomly on separate 4x4 grids can be rotated (without reflection) to match each other, follow this algorithmic recipe described in prose. The L-tetromino is a shape consisting of four squares connected edge-to-edge, forming an "L" shape. Since rotations are allowed but reflections are not, we need to check if one tetromino can be transformed into the other through a sequence of 90-degree rotations (0°, 90°, 180°, or 270°). Here’s how to proceed step by step:

1. **Identify the Positions of the Tetrominoes on Each Grid**:  
   Start by examining the two 4x4 grids and recording the coordinates of the four squares occupied by the L-tetromino on each grid. Label the grids as Grid A and Grid B. For each grid, create a list of the (row, column) positions of the tetromino’s squares. Since the grids are 4x4, row and column indices range from 0 to 3. Ensure that each tetromino consists of exactly four connected squares forming an L shape.

2. **Normalize the Position of Each Tetromino**:  
   To simplify comparison, translate each tetromino so that its "reference point" is at a standard location, such as the top-leftmost square of the tetromino. For each tetromino, find the smallest row index and smallest column index among its four squares. Subtract these minimum row and column values from all the coordinates of the tetromino’s squares to shift it to the origin (0,0) or as close as possible. This step ensures that the relative shape is preserved while removing differences due to absolute positioning on the grid. Record the normalized set of coordinates for the tetrominoes on Grid A and Grid B.

3. **Define the Canonical Orientations of an L-Tetromino**:  
   Recognize that an L-tetromino has four distinct orientations under rotation (no reflection). Define a standard set of coordinates for each orientation relative to a reference point (e.g., after normalization). For example:  
   - Orientation 0 (standard L): [(0,0), (0,1), (0,2), (1,0)]  
   - Orientation 1 (rotated 90° clockwise): [(0,0), (0,1), (1,1), (2,1)]  
   - Orientation 2 (rotated 180°): [(0,2), (1,0), (1,1), (1,2)]  
   - Orientation 3 (rotated 270° clockwise): [(0,0), (1,0), (2,0), (2,1)]  
   These represent the four possible orientations of an L-tetromino after normalization. Ensure these coordinate sets are consistent with your normalization method.

4. **Determine the Orientation of Tetromino A**:  
   Compare the normalized coordinates of the tetromino on Grid A with the four canonical orientations defined above. Identify which orientation matches the normalized coordinates of Tetromino A. If it matches one of the orientations, note the corresponding rotation (0°, 90°, 180°, or 270°). If it does not match any (which should not happen if it’s a valid L-tetromino), there is an error in the input or process.

5. **Check All Possible Rotations of Tetromino B**:  
   Since we want to see if Tetromino B can be rotated to match Tetromino A, take the normalized coordinates of Tetromino B and apply rotations of 0°, 90°, 180°, and 270° to its coordinates. To rotate a set of coordinates by 90° clockwise around the origin, transform each (x,y) to (y, -x). Adjust for the grid size if needed, but since we normalized to near (0,0), this should be straightforward. For each rotation:  
   - Compute the new coordinates after rotation.  
   - Compare the rotated coordinates of Tetromino B with the normalized coordinates of Tetromino A.  
   - If any rotated set of coordinates matches Tetromino A’s coordinates exactly (same set of points), then Tetromino B can be rotated to match Tetromino A.

6. **Conclude the Result**:  
   If any of the rotated versions of Tetromino B match Tetromino A, output that it is possible to rotate one tetromino to match the other. Specifically, note that a rotation exists (corresponding to the difference in orientation between the matched rotation of B and the original orientation of A). If none of the rotations of Tetromino B match Tetromino A, conclude that it is not possible to rotate one to match the other. Since reflections are not allowed and L-tetrominoes have distinct orientations under rotation only, this process will correctly determine the result.

7. **Handle Edge Cases and Validation**:  
   Before concluding, ensure that the input tetrominoes are valid L-tetrominoes by verifying that each consists of four connected squares forming an L shape. If the tetrominoes are not valid or if they are reflections of each other (which cannot be matched by rotation alone), the algorithm should detect this during the comparison step and return that no rotation can match them. Additionally, since the grid is 4x4, ensure that rotations do not result in coordinates outside the grid bounds if normalization is adjusted; however, since normalization minimizes this issue, it should not be a problem.

By following this recipe, you can systematically determine whether one L-tetromino can be rotated to match the other on separate 4x4 grids. The process accounts for all possible orientations through rotation and ensures that reflections are not considered, adhering to the problem constraints.

