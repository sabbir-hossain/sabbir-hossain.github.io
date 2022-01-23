let currentPage = 0;

window.addEventListener("load", function(event) {
  currentPage = 0;
  draw_animation(currentPage);
});

document.getElementById("backwardDiv").addEventListener("click", evt => {
  evt.preventDefault();
  currentPage--;

  if (currentPage >= 0 && currentPage < data.length) {
    draw_animation(currentPage);
  }

  if (currentPage < 0) {
    currentPage = data.length - 1;
    draw_animation(currentPage);
  }
});

document.getElementById("forwardDiv").addEventListener("click", evt => {
  evt.preventDefault();
  currentPage++;
  if (currentPage >= 0 && currentPage < data.length) {
    draw_animation(currentPage);
  }

  if (currentPage >= data.length) {
    currentPage = 0;
    draw_animation(currentPage);
  }
});

function draw_animation(currentPage) {
  // get a reference to the canvas
  const canvas_id = "myCanvas";
  const canvas = document.getElementById(canvas_id);
  const ctx = canvas.getContext("2d");

  const clientWidth = document.getElementById('canvas-area').clientWidth;

  canvas.width = clientWidth;
  canvas.height = Math.round(canvas.width * 0.75);

  // get a reference to the drawing context
  const bgCanvas = document.createElement("canvas");
  // const bgContext = bgCanvas.getContext("2d").canvas;

  let startY = 50;
  let startX = 0;
  const { text = "", shadowText = [], displayRatio = 0 } =
    data[currentPage] || {};
  
  // displayAnimation(canvas, ctx, startX, startY);

  displayAnimation(canvas, ctx, text, shadowText, displayRatio, startX, startY);
}
