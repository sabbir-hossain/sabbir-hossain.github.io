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
  // ctx.textAlign = "end";

  /// color for background
  // ctx.fillStyle = "#f50";
  ctx.fillStyle = color;

  // get width of text
  var width = ctx.measureText(txt).width;

  // draw background rect assuming height of font
  ctx.fillRect(x, y, minWidth, parseInt(font, 10));

  /// text color
  // ctx.fillStyle = color;
  // ctx.fillStyle = "#000";
  ctx.fillStyle = "#f9f9f9";

  /// draw text on top
  ctx.fillText(txt, x, y);
  // ctx.strokeText(txt, x, y);

  /// restore original state
  ctx.restore();
}
