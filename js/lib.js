Array.prototype.random = function() {
  return this[Math.round(Math.random() * (this.length - 1))];
};

// expand with color, background etc.
function drawTextBG(ctx, txt, font, x, y, color) {
  const minWidth = 27;
  /// lets save current state as we make a lot of changes
  ctx.save();

  /// set font
  ctx.font = `${font}px Calibri`;

  /// draw text from top - makes life easier at the moment
  ctx.textBaseline = "top";

  /// color for background
  // ctx.fillStyle = "#f50";
  ctx.fillStyle = color;

  // get width of text
  var width = ctx.measureText(txt).width;

  // draw background rect assuming height of font
  ctx.fillRect(x, y, minWidth, parseInt(font, 10));
  // drawBorder(ctx, x, y, minWidth, parseInt(font, 10), color, (thickness = 1));

  /// text color
  // ctx.fillStyle = color;
  // ctx.fillStyle = "#000";
  ctx.fillStyle = "#f9f9f9";

  /// draw text on top
  ctx.fillText(txt === "M" || txt === "m" ? txt : ` ${txt} `, x, y);
  // ctx.strokeText(txt, x, y);

  /// restore original state
  ctx.restore();
}

function drawBorder(ctx, xPos, yPos, width, height, color, thickness = 1) {
  ctx.fillStyle = color;
  ctx.fillRect(
    xPos - thickness,
    yPos - thickness,
    width + thickness * 2,
    height + thickness * 2
  );
}

const colorList = ["orange", "green", "red", "blue", "indigo"];

const font = 30;
const line = 30;
const minWidth = 27;
