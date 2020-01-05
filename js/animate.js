window.addEventListener("load", function(event) {
  draw_animation();
});

function draw_animation() {
  // get a reference to the canvas
  const canvas_id = "myCanvas";
  const canvas = document.getElementById(canvas_id);
  const ctx = canvas.getContext("2d");

  const canvasWidth = window.innerWidth * 0.75;
  console.log({ canvasWidth, val: canvasWidth - 100 });

  canvas.width = canvasWidth;
  canvas.height = window.innerHeight - 100;

  // get a reference to the drawing context
  const bgCanvas = document.createElement("canvas");
  const bgContext = bgCanvas.getContext("2d").canvas;

  let startY = 50;
  let startX = 0;

  displayAnimation(ctx, startX, startY);
}
