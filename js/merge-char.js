const charHeight = 7;

function scaleUpAllCharList(charList, displayRatio, schemaCharLen) {
  const margeCharList = [];
  const multiplyWithRatio = Math.floor(displayRatio * schemaCharLen);
  const firstPortion = schemaCharLen - multiplyWithRatio;

  for (let i = 0; i < schemaCharLen; i++) {
    margeCharList[i] = [];
    let portion = 0;
    if (i < firstPortion) {
      portion = 1;
    } else {
      portion = 2;
    }

    for (let j = 0, len = charList.length; j < len; j++) {
      margeCharList[i].push(
        ...scaleUpCharColumn(charSchemaList[charList[j]][i], portion)
      );
      margeCharList[i].push(0);
    }
  }
  return margeCharList;
}

function scaleUpCharColumn(info = [], portion) {
  const data = [];
  for (let i = 0, len = info.length; i < len; i++) {
    data.push(`${info[i]}_${portion}`);
  }
  return data;
}

function scaleUpCharAgain(result, inputTextArr, maxChar) {
  const line = result.length;
  const inputLine = inputTextArr.length;

  const resultCharLen = result[0].length;
  const inputCharLen = inputTextArr[0].length;

  const limit2 = Math.round((inputCharLen - resultCharLen) / 2);

  for (let i = 0; i < line; i++) {
    for (let j = 0; j < limit2; j++) {
      result[i].unshift(0);
    }
  }

  const limit = Math.round((inputLine - line) / 2);
  for (let i = 0; i < limit - 1; i++) {
    const arr = Array(maxChar).fill(0);
    result.unshift(arr);
  }

  return result;
}
