# marching_squares

<img width="1392" alt="Screenshot 2023-05-29 at 11 22 18" src="https://github.com/Dot32IsCool/marching_squares/assets/61964090/8eb7888a-b476-453f-99d3-7d51b26fa85f">

Marching squares implemented in Bevy, with a mesh that updates every frame! Egui is used for an interactive settings menu.

If you would like to try this yourself, rather than trying to read my pile of spaghetti code, I recommend watching [this video](https://www.youtube.com/watch?v=0ZONMNUKTfU) 
by The Coding Train.
When it comes to implementing Lerping (which is not covered in the video), you can use an [inverse lerp](https://www.gamedev.net/articles/programming/general-and-gameplay-programming/inverse-lerp-a-super-useful-yet-often-overlooked-function-r5230/)
function with your two points' values as a and b, and your threshold as the "value" argument. It will return a `t` value, representing how far along the threshold is between your two values, that you can then use to move mesh points around. 
