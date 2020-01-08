function displayAnimation(ctx, inputStr, charList, displayRatio, x, y) {
  const colorData = getColorData(charList);
  const { schemaCharLen } = getSchemaData(charList);

  let result = scaleUpAllCharList(charList, displayRatio, schemaCharLen);

  const totalChar = inputStr.length;
  const cW = ctx.canvas.width;
  const cH = ctx.canvas.height;

  const total = (cW - x * 2) * (cH - y * 2);
  const val = total / totalChar;
  let width = Math.round(Math.sqrt(val) * 0.8);

  let maxChar = Math.round(cW / width);

  if (result[0].length > maxChar) {
    const prevMaxChar = maxChar;
    maxChar = result[0].length;
    width = Math.round((prevMaxChar * width) / maxChar) - 1;
    shouldUpdate = true;
  }

  let height = Math.round(width * 0.65);

  const inputTextArr = convertStringTo2DArray(inputStr, maxChar, result);
  result = scaleUpCharAgain(result, inputTextArr, maxChar);

  const animateInterval = setInterval(
    () => animate(ctx, inputTextArr, result, colorData, x, y, width, height),
    85
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
        let textColor = "#f9f9f9";
        let fontStyle = "";

        if (
          typeof result[i] !== "undefined" &&
          typeof colorData[result[i][j]] !== "undefined"
        ) {
          color = colorData[result[i][j]];
          textColor = "#333";
          fontStyle = "bold";
        }

        const input = inputTextArr.random();
        const val = input.random();
        drawTextBG(
          ctx,
          val,
          startX,
          startY,
          color,
          width,
          height,
          textColor,
          fontStyle
        );
        // drawTextBG(ctx, txt, x, y, color, width, height, textColor)
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
      let textColor = "#f9f9f9";
      let fontStyle = "";
      if (
        typeof result[i] !== "undefined" &&
        typeof colorData[result[i][j]] !== "undefined"
      ) {
        color = colorData[result[i][j]];
        textColor = "#333";
        fontStyle = "bold";
      }

      drawTextBG(
        ctx,
        inputTextArr[i][j],
        startX,
        startY,
        color,
        width,
        height,
        textColor,
        fontStyle
      );
      startX += width;
    }
    startY += height;
  }
}
