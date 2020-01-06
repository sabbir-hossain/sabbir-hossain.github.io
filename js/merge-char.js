const charHeight = 7;

function scaleUpAllCharList(
  charList,
  displayRatio,
  { maxLine, maxChar },
  { schemaCharLen, schemaCharBitLen }
) {
  const margeCharList = mergeAllCharList(charList, schemaCharLen);
  console.log({ maxChar, margeCharList, v: margeCharList[0].length });
  const scaleX = Math.floor(maxChar / margeCharList[0].length);
  const scaleY = Math.floor(maxLine / schemaCharLen);

  const data = [];
  let counter = 0;
  const ratio = Math.ceil(scaleY / scaleX);

  // const scaleY = Math.floor(maxChar / charLen);
  const inScaleY = maxLine % schemaCharLen;
  const st = maxLine - inScaleY;
  const div = Math.floor(st / 2);

  const multiplyWithRatio = Math.floor(displayRatio * maxLine);
  const firstPortion = maxLine - multiplyWithRatio;
  const secondPortion = maxLine - firstPortion;
  console.log({ scaleX, scaleY, inScaleY, firstPortion });
  for (let i = 0, len = margeCharList.length; i < len; i++) {
    let portion = 0;
    if (i < firstPortion) {
      portion = 1;
    } else {
      portion = 2;
    }
    for (let j = 0; j < scaleY; j++) {
      const d = scaleUpCharColumn(margeCharList[i], portion, {
        maxLine,
        maxChar
      });
      data[counter] = [];
      data[counter].push(...d);
      counter++;
    }

    if (div < i && i < maxLine - div) {
      const d = scaleUpCharColumn(margeCharList[i], portion, {
        maxLine,
        maxChar
      });
      data[counter] = [];
      data[counter].push(...d);
      counter++;
    }
  }
  return data;
}

function mergeAllCharList(charList, schemaCharLen) {
  const margeCharList = [];

  for (let i = 0; i < schemaCharLen; i++) {
    margeCharList[i] = [];
    for (let j = 0, len = charList.length; j < len; j++) {
      margeCharList[i].push(...charSchemaList[charList[j]][i]);
      margeCharList[i].push(0);
      margeCharList[i].push(0);
    }
  }
  return margeCharList;
}

function scaleUpCharColumn(info = [], portion, { maxLine, maxChar }) {
  const data = [];
  const charLen = info.length;
  const scaleX = Math.floor(maxChar / charLen);
  const inScaleX = maxChar % charLen;
  const st = charLen - inScaleX;
  const div = Math.floor(st / 2);

  for (let i = 0, len = info.length; i < len; i++) {
    for (let j = 0; j < scaleX; j++) {
      data.push(`${info[i]}_${portion}`);
    }

    if (div <= i && i < charLen - div) {
      data.push(`${info[i]}_${portion}`);
    }
  }
  return data;
}
