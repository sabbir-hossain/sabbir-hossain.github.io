const inputText = [
  [
    "H",
    "i",
    ",",
    "I",
    " ",
    "a",
    "m",
    " ",
    "S",
    "a",
    "b",
    "b",
    "i",
    "r",
    ".",
    " ",
    "A",
    " ",
    "v",
    "e",
    "r",
    "y"
  ],
  [
    " ",
    "o",
    "r",
    "i",
    "d",
    "i",
    "n",
    "a",
    "r",
    "y",
    " ",
    "f",
    "u",
    "l",
    "l",
    "s",
    "t",
    "a",
    "c",
    "k",
    " ",
    " "
  ],
  [
    "d",
    "e",
    "v",
    "e",
    "l",
    "o",
    "p",
    "e",
    "r",
    ".",
    "  ",
    " ",
    " ",
    " ",
    " ",
    " ",
    " ",
    " ",
    " ",
    " ",
    " ",
    " "
  ],
  [
    "M",
    "y",
    " ",
    "w",
    "o",
    "r",
    "s",
    "t",
    " ",
    "n",
    "i",
    "g",
    "h",
    "t",
    "m",
    "a",
    "r",
    "e",
    " ",
    "i",
    "s",
    " "
  ],
  [
    "t",
    "h",
    "a",
    "t",
    " ",
    "s",
    "o",
    "m",
    "e",
    "o",
    "n",
    "e",
    " ",
    "t",
    "o",
    "l",
    "d",
    " ",
    "m",
    "e",
    " ",
    "s"
  ],
  [
    "o",
    " ",
    "y",
    "o",
    "u",
    " ",
    "a",
    "r",
    "e",
    " ",
    "s",
    "o",
    "f",
    "t",
    "w",
    "a",
    "r",
    "e",
    " ",
    "e",
    "n",
    "g"
  ],
  [
    "i",
    "n",
    "e",
    "e",
    "r",
    ",",
    " ",
    "c",
    "a",
    "n",
    " ",
    "y",
    "o",
    "u",
    " ",
    "f",
    "i",
    "x",
    " ",
    "m",
    "y",
    " "
  ],
  [
    "s",
    "m",
    "a",
    "r",
    "t",
    "p",
    "h",
    "o",
    "n",
    "e",
    "/",
    "c",
    "o",
    "m",
    "p",
    "u",
    "t",
    "e",
    "r",
    "?",
    " ",
    " "
  ]
];

function displayAnimation(ctx, x, y) {
  const maxLine = inputText.length;
  const maxChar = inputText[0].length;

  const displayRatio = 0.9;

  const charList = ["J", "S"];

  // charSchemaProsObj
  const colorData = charList.reduce((total, current) => {
    total[`${charSchemaProsObj[current]}_1`] =
      colorList[Math.floor(Math.random() * colorList.length)];

    total[`${charSchemaProsObj[current]}_2`] =
      colorList[Math.floor(Math.random() * colorList.length)];
    return total;
  }, {});

  let schemaCharBitLen = 0;
  let schemaCharLen = 0;
  for (let i = 0, lenx = charList.length; i < lenx; i++) {
    schemaCharLen = charSchemaList[charList[i]].length;
    schemaCharBitLen += charSchemaList[charList[i]][0].length;
  }

  const result = scaleUpAllCharList(
    inputText,
    charList,
    displayRatio,
    {
      maxLine,
      maxChar
    },
    { schemaCharLen, schemaCharBitLen }
  );

  let counter = 0;
  const animateInterval = setInterval(
    () => animate(ctx, result, colorData, x, y, counter),
    70
  );

  setTimeout(function() {
    clearInterval(animateInterval);
    drawFixedText(ctx, result, colorData, x, y);
  }, 1000 * 2);

  function animate(ctx, result, colorData, x, y) {
    ctx.clearRect(0, 0, ctx.canvas.width, ctx.canvas.height);
    ctx.save();
    let startY = y;
    for (let i = 0, len = inputText.length; i < len; i++) {
      let startX = x;
      for (let j = 0, len2 = inputText[i].length; j < len2; j++) {
        let color = "#222";
        if (typeof colorData[result[i][j]] !== "undefined") {
          color = colorData[result[i][j]];
        }

        const input = inputText.random();
        const val = input.random();
        drawTextBG(ctx, val, font, startX, startY, color);
        // startX += minWidth + 5;
        startX += minWidth;
      }
      // startY += line + 7;
      startY += line;
    }
  }
  ctx.restore();
}

function drawFixedText(ctx, result, colorData, x, y) {
  ctx.clearRect(0, 0, ctx.canvas.width, ctx.canvas.height);
  let startY = y;
  for (let i = 0, len = inputText.length; i < len; i++) {
    let startX = x;
    for (let j = 0, len2 = inputText[i].length; j < len2; j++) {
      let color = "#222";
      if (typeof colorData[result[i][j]] !== "undefined") {
        color = colorData[result[i][j]];
      }

      drawTextBG(ctx, inputText[i][j], font, startX, startY, color);
      // startX += minWidth + 5;
      startX += minWidth;
    }
    // startY += line + 7;
    startY += line;
  }
}
