# Raytracer
Heavily inspired from https://raytracing.github.io/, book 1 and 2.

For changing parameters such as width, height, ray depth or the number of workers, you have to modify the code directly.
Make sure to run in release mode to avoid having lots of logs printed in stderr.
When the image is fully displayed, hit S to save in "image.png" in the current directory.

For choosing the scene, set the environment variable **SCENE** to:

* **MARBLE** if you want this:
![A marble sphere on top on a marble plane.](./images/marble.png)
There is currently a problem with MARBLE, I don't remembered what parameters I had to get this beautiful picture, sorry 🥲

* **SPHERES** if you want this:
![Two spheres, one on top of the others, so big you can't see them totally, with a green and white checker texture.](./images/spheres.png)

* **RANDOM** if you want this:
![Three big spheres on a green and white checker-textured plane, the first one is made of glass, the second of metal and the last one of a lambertian material. There are a lot of moving or fixed small lambertian, metal of glass spheres around.](./images/random.png)
The black point are points where the max depth is too small. Blur represents "moving" balls, though it may too pronounced sometimes.
