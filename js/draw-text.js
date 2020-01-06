const inputStr = `Hi, I am sabbir. A very ordinary Full-Stack application developer.\n
As a software engineer, My worst nightmare is that, one day someone told me,\n
"So you are software engineer, can you fix my smartphone/computer?"`;

function displayAnimation(ctx, inputStr, charList, displayRatio, x, y) {
  const colorData = getColorData(charList);

  const totalChar = inputStr.length;
  const cW = ctx.canvas.width;
  const cH = ctx.canvas.height;

  const total = (cW - x * 2) * (cH - y * 2);
  const val = total / totalChar;
  let width = Math.round(Math.sqrt(val) * 0.8);
  width = width > 1000 ? 1000 : width;
  // const height = Math.round(width * 0.75);
  const height = width;
  const maxChar = Math.round(cW / width);
  let maxLine = Math.ceil(totalChar / maxChar);
  console.log({ totalChar, maxLine, maxChar });
  const inputTextArr = convertStringTo2DArray(inputStr, maxChar);

  const { schemaCharBitLen, schemaCharLen } = getSchemaData(charList);
  console.log({ schemaCharBitLen, schemaCharLen, colorData });
  maxLine = inputTextArr.length;

  const result = scaleUpAllCharList(
    charList,
    displayRatio,
    {
      maxLine,
      maxChar
    },
    { schemaCharLen, schemaCharBitLen }
  );
  console.log({ result });
  const animateInterval = setInterval(
    () => animate(ctx, inputTextArr, result, colorData, x, y, width, height),
    75
  );

  setTimeout(function() {
    clearInterval(animateInterval);
    drawFixedText(ctx, inputTextArr, result, colorData, x, y, width, height);
  }, 500);

  function animate(ctx, inputTextArr, result, colorData, x, y, width, height) {
    ctx.clearRect(0, 0, ctx.canvas.width, ctx.canvas.height);
    ctx.save();
    let startY = y;

    for (let i = 0, len = inputTextArr.length; i < len; i++) {
      let startX = x;
      for (let j = 0, len2 = inputTextArr[i].length; j < len2; j++) {
        let color = "#333";
        // if (typeof colorData[result[i][j]] !== "undefined") {
        //   color = colorData[result[i][j]];
        // }

        if (
          typeof result[i] !== "undefined" &&
          typeof colorData[result[i][j]] !== "undefined"
        ) {
          color = colorData[result[i][j]];
        }

        const input = inputTextArr.random();
        const val = input.random();
        drawTextBG(ctx, val, startX, startY, color, width, height);
        startX += width;
      }
      startY += height;
    }
  }
  ctx.restore();
}

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
      if (
        typeof result[i] !== "undefined" &&
        typeof colorData[result[i][j]] !== "undefined"
      ) {
        color = colorData[result[i][j]];
      }

      drawTextBG(ctx, inputTextArr[i][j], startX, startY, color, width, height);
      startX += width;
    }
    startY += height;
  }
}
