const charSchemaList = {
  A: [
    [0, 0, 1, 0, 0],
    [0, 1, 0, 1, 0],
    [1, 0, 0, 0, 1],
    [1, 1, 1, 1, 1],
    [1, 0, 0, 0, 1]
  ],
  B: [
    [2, 2, 2, 2, 0],
    [2, 0, 0, 0, 2],
    [2, 0, 0, 0, 2],
    [2, 2, 2, 2, 0],
    [2, 0, 0, 0, 2],
    [2, 0, 0, 0, 2],
    [2, 2, 2, 2, 0]
  ],
  I: [[9], [9], [9], [9], [9], [9], [9]],
  J: [
    [10, 10, 10, 10, 10],
    [0, 0, 0, 0, 10],
    [0, 0, 0, 0, 10],
    [0, 0, 0, 0, 10],
    [10, 0, 0, 0, 10],
    [10, 0, 0, 0, 10],
    [10, 10, 10, 10, 10]
  ],
  S: [
    [19, 19, 19, 19, 19],
    [19, 0, 0, 0, 19],
    [19, 0, 0, 0, 0],
    [19, 19, 19, 19, 19],
    [0, 0, 0, 0, 19],
    [19, 0, 0, 0, 19],
    [19, 19, 19, 19, 19]
  ],
  R: [
    [18, 18, 18, 18, 0],
    [18, 0, 0, 0, 18],
    [18, 0, 0, 0, 18],
    [18, 18, 18, 18, 0],
    [18, 0, 18, 0, 0],
    [18, 0, 18, 0, 0],
    [18, 0, 0, 18, 0]
  ]
};

const charSchemaProsObj = {
  A: 1,
  B: 2,
  I: 9,
  J: 10,
  R: 18,
  S: 19
};

function getSchemaData(charList) {
  let schemaCharBitLen = 0;
  let schemaCharLen = 0;
  for (let i = 0, lenx = charList.length; i < lenx; i++) {
    schemaCharLen = charSchemaList[charList[i]].length;
    schemaCharBitLen += charSchemaList[charList[i]][0].length;
  }

  return { schemaCharBitLen, schemaCharLen };
}
