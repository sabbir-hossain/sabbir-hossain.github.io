Array.prototype.random = function() {
  return this[Math.round(Math.random() * (this.length - 1))];
};

const colorList = ["orange", "green", "red", "blue", "indigo"];

// expand with color, background etc.
function drawTextBG(
  ctx,
  txt,
  x,
  y,
  color,
  width,
  height,
  textColor = "#f9f9f9"
) {
  /// lets save current state as we make a lot of changes
  ctx.save();

  /// set font
  ctx.font = `${height}px Calibri`;

  /// draw text from top - makes life easier at the moment
  ctx.textBaseline = "top";

  /// color for background
  // ctx.fillStyle = "#f50";
  ctx.fillStyle = color;

  // get width of text
  var wd = ctx.measureText(txt).width;

  // draw background rect assuming height of font
  ctx.fillRect(x, y, width, parseInt(height, 10));
  // drawBorder(ctx, x, y, minWidth, parseInt(font, 10), color, (thickness = 1));

  /// text color
  // ctx.fillStyle = color;
  // ctx.fillStyle = "#000";
  ctx.fillStyle = textColor;

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

function convertStringTo2DArray(inputStr, maxChar) {
  const inputcharList = inputStr.split("");
  const inputTextArr = [];
  let st = 0;
  let counter = 0;
  let isRunning = true;

  while (isRunning) {
    inputTextArr[counter] = [];
    let newLineFound = false;
    const len = st + maxChar;
    let j = st;
    for (; j < len; j++) {
      if (
        typeof inputcharList[j] === "undefined" ||
        inputcharList[j] === undefined
      ) {
        inputTextArr[counter].push(" ");
      } else if (inputcharList[j] !== "\n") {
        inputTextArr[counter].push(inputcharList[j]);
      } else {
        newLineFound = true;
        break;
      }
    }

    if (newLineFound) {
      for (let k = j; k < len; k++) {
        inputTextArr[counter].push(" ");
      }
      counter++;
      inputTextArr[counter] = Array(maxChar).fill(" ");
      st = j + 2;
      counter++;
    } else {
      st += maxChar;
      counter++;
    }

    if (typeof inputcharList[j] === "undefined") {
      isRunning = false;
    }
  }

  return inputTextArr;
}

function getColorData(charList) {
  return charList.reduce((total, current) => {
    total[`${charSchemaProsObj[current]}_1`] = colorList.random();

    total[`${charSchemaProsObj[current]}_2`] = colorList.random();
    return total;
  }, {});
}
