window.addEventListener("load", function(event) {
  draw_animation();
});

function draw_animation() {
  // get a reference to the canvas
  const canvas = document.getElementById("myCanvas");

  // get a reference to the drawing context
  const ctx = canvas.getContext("2d");

  const cW = ctx.canvas.width;
  const cH = ctx.canvas.height;

  let startY = 50;
  let startX = 50;

  displayAnimation(ctx, startX, startY);
}
