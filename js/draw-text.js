const inputText = [
  [
    "H",
    "i",
    ",",
    " ",
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
    "y",
    " ",
    "o",
    "r",
    "i",
    "d",
    "-"
  ],
  [
    "i",
    "n",
    "a",
    "r",
    "y",
    " ",
    "F",
    "u",
    "l",
    "l",
    "-",
    "S",
    "t",
    "a",
    "c",
    "k",
    " ",
    "a",
    "p",
    "p",
    "l",
    "i",
    "c",
    "a",
    "t",
    "i",
    "o",
    "n",
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
    "A",
    "s",
    " ",
    "a",
    " ",
    "S",
    "o",
    "f",
    "t",
    "w",
    "a",
    "r",
    "e",
    " ",
    "E",
    "n",
    "g",
    "i",
    "n",
    "e",
    "e",
    "r",
    ",",
    " ",
    "M",
    "y",
    " ",
    "w",
    "-"
  ],
  [
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
    " ",
    "t",
    "h",
    "a",
    "t",
    ",",
    " ",
    "s",
    "o",
    "m",
    "e",
    "-"
  ],
  [
    "d",
    "a",
    "y",
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
    ",",
    " ",
    '"',
    "s",
    "o",
    " ",
    "y",
    "o",
    "u",
    " "
  ],
  [
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
    "g",
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
    "-"
  ],
  [
    "o",
    "u",
    " ",
    "f",
    "i",
    "x",
    " ",
    "t",
    "h",
    "i",
    "s",
    " ",
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
    "-"
  ],
  [
    "t",
    "e",
    "r",
    "?",
    '"',
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
    " ",
    " ",
    " ",
    " "
  ]
];

const inputStr = `Hi, I am sabbir. A very ordinary Full-Stack application developer.\r\n
  As a software engineer, My worst nightmare is that, one day someone told me,\r\n
  "So you are software engineer, can you fix my smartphone/computer?"\r\n
  `;

function displayAnimation(ctx, x, y) {
  const displayRatio = 0.9;

  const charList = ["J", "S"];

  const totalChar = inputStr.length;
  const cW = ctx.canvas.width;
  const cH = ctx.canvas.height;

  const total = (cW - x * 2) * (cH - y * 2);
  const val = total / totalChar;
  const width = Math.round(Math.sqrt(val) * 0.8);
  // const height = Math.round(width * 0.75);
  const height = width;
  const maxChar = Math.round(cW / width);
  const maxLine = Math.ceil(totalChar / maxChar);
  console.log({ width, height });

  const inputcharList = inputStr.split("");
  const inputTextArr = [];
  let st = 0;
  for (let i = 0; i < maxLine; i++) {
    const strArr = inputcharList.slice(st, st + maxChar);
    inputTextArr.push(strArr);
    st += maxChar;
  }

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
    inputTextArr,
    charList,
    displayRatio,
    {
      maxLine,
      maxChar
    },
    { schemaCharLen, schemaCharBitLen }
  );

  const animateInterval = setInterval(
    () => animate(ctx, result, colorData, x, y, width, height),
    70
  );

  setTimeout(function() {
    clearInterval(animateInterval);
    drawFixedText(ctx, inputTextArr, result, colorData, x, y, width, height);
  }, 500);

  function animate(ctx, result, colorData, x, y, width, height) {
    ctx.clearRect(0, 0, ctx.canvas.width, ctx.canvas.height);
    ctx.save();
    let startY = y;
    for (let i = 0, len = inputTextArr.length; i < len; i++) {
      let startX = x;
      for (let j = 0, len2 = inputTextArr[i].length; j < len2; j++) {
        let color = "#333";
        if (typeof colorData[result[i][j]] !== "undefined") {
          color = colorData[result[i][j]];
        }

        const input = inputTextArr.random();
        const val = input.random();
        drawTextBG(ctx, val, font, startX, startY, color, width, height);
        // startX += minWidth + 5;
        startX += width;
      }
      // startY += line + 7;
      startY += height;
    }
  }
  ctx.restore();
}

/*
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

  const animateInterval = setInterval(
    () => animate(ctx, result, colorData, x, y),
    70
  );

  setTimeout(function() {
    clearInterval(animateInterval);
    drawFixedText(ctx, result, colorData, x, y);
  }, 500);

  function animate(ctx, result, colorData, x, y) {
    ctx.clearRect(0, 0, ctx.canvas.width, ctx.canvas.height);
    ctx.save();
    let startY = y;
    for (let i = 0, len = inputText.length; i < len; i++) {
      let startX = x;
      for (let j = 0, len2 = inputText[i].length; j < len2; j++) {
        let color = "#333";
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
*/

function drawFixedText(
  ctx,
  inputTextArr,
  result,
  colorData,
  x,
  y,
  width,
  height
) {
  ctx.clearRect(0, 0, ctx.canvas.width, ctx.canvas.height);
  let startY = y;
  for (let i = 0, len = inputTextArr.length; i < len; i++) {
    let startX = x;
    for (let j = 0, len2 = inputTextArr[i].length; j < len2; j++) {
      let color = "#333";
      if (typeof colorData[result[i][j]] !== "undefined") {
        color = colorData[result[i][j]];
      }

      drawTextBG(
        ctx,
        inputTextArr[i][j],
        font,
        startX,
        startY,
        color,
        width,
        height
      );
      // startX += minWidth + 5;
      startX += width;
    }
    // startY += line + 7;
    startY += height;
  }
}
