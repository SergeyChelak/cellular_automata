# Cellular automata

## Overview
This research project explores the generation of intricate maze structures tailored for first person shooter (FPS) game environments. Utilizing cellular automata logic, the project aims to create dynamic and challenging level maps while offering parameterization options for initial noise density and the number of iterations (generations). This customization enables researchers to fine-tune maze characteristics and study their impact on gameplay experiences.

## Getting Started
- Rust programming language
- SDL2 library with image features

## Controls
| key      | description |
| -------- | ----------- |
| q          | increase noise density  |
| a          | decrease noise density  |
| w          | increase iterations |
| s          | decrease iterations |
| n          | next iteration with current matrix  |
| r          | regenerate using current settings   |
| f          | filter small regions    |
| m          | perform full cycle maze generation  |
| esc        | exit    |
| other key  | display generator settings  |

## Processing
Note: App processing with two logical values: "floor" and "wall"
1. Generate noise (random) matrix with specified density
2. Create new matrix by applying cellular automata for each cell
3. Repeat step 2 specified amount of steps
4. Find with BFS all "floor" regions
5. Threat all unreachable (isolated) regions as a "wall"
6. Find with BFS all "wall" regions
7. Threat all small (depends on threshold) wall regions as a "floor"
6. Calculate contours

## Example
This example demonstrates connected "floor" area which is isolated with a large "wall" region. Also there're presented smaller "wall" regions as part of maze. Contour cells are highlight with a lighter color.
![Image](docs/example.png)

## P.S.
Anyone can use this code and described approach without any limitations. If this project was helpful for you let me know how did you apply it