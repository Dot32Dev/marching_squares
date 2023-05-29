# marching_squares

<img width="1392" alt="Screenshot 2023-05-29 at 11 22 18" src="https://github.com/Dot32IsCool/marching_squares/assets/61964090/8eb7888a-b476-453f-99d3-7d51b26fa85f">

Marching squares implemented in Bevy, with a mesh that updates every frame! Egui is used for an interactive settings menu.

## If you would like to try making this yourself...
Rather than trying to read my pile of spaghetti code, I recommend watching [this video](https://www.youtube.com/watch?v=0ZONMNUKTfU) 
by The Coding Train.
When it comes to implementing Lerping (which is not covered in the video), you can use an [inverse lerp](https://www.gamedev.net/articles/programming/general-and-gameplay-programming/inverse-lerp-a-super-useful-yet-often-overlooked-function-r5230/)
function with your two points' values as a and b, and your threshold as the "value" argument. It will return a `t` value, representing how far along the threshold is between your two values. You can then use this to move mesh points around. 

I found [this project](https://github.com/JosePedroDias/rust_experiments/blob/main/bevy/src/shapes/circle.rs) useful in learning procedural mesh generation in general. The "positions" describe all of the vertices, and the indicies describe which vertices are used to create each triangle. This allows rendering a quad with only 4 vertices instead of 6, for example, as some vertices are reused. When it came to spawning the mesh, I followed [this bevy example](https://bevyengine.org/examples-webgpu/2D%20Rendering/mesh2d/) that spawned a default quad mesh. 

If you want to update the mesh, remember to query for a `Mesh2dHandle` instead of a `Handle<Mesh>`, this just being an annoying inconsistency between 2D and 3D in Bevy.