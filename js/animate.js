let currentPage = 0;

window.addEventListener("load", function(event) {
  currentPage = 1;
  draw_animation(currentPage);
});

document.getElementById("backwareDiv").addEventListener("click", evt => {
  evt.preventDefault();
  currentPage--;

  if (currentPage >= 0 && currentPage < data.length) {
    draw_animation(currentPage);
  }
});

document.getElementById("forwardDiv").addEventListener("click", evt => {
  evt.preventDefault();
  currentPage++;
  if (currentPage >= 0 && currentPage < data.length) {
    draw_animation(currentPage);
  }
});

function draw_animation(currentPage) {
  // get a reference to the canvas
  const canvas_id = "myCanvas";
  const canvas = document.getElementById(canvas_id);
  const ctx = canvas.getContext("2d");

  const canvasWidth = window.innerWidth * 0.75;

  canvas.width = canvasWidth;
  canvas.height = window.innerHeight - 100;

  // get a reference to the drawing context
  const bgCanvas = document.createElement("canvas");
  const bgContext = bgCanvas.getContext("2d").canvas;

  let startY = 50;
  let startX = 0;
  const { text = "", shadowText = [], displayRatio = 0 } =
    data[currentPage] || {};
  // displayAnimation(ctx, startX, startY);
  console.log({
    text,
    shadowText,
    startX,
    startY,
    currentPage,
    d: data[currentPage]
  });
  displayAnimation(ctx, text, shadowText, displayRatio, startX, startY);
}
