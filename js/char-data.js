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
  D: [
    [4, 4, 4, 0],
    [4, 0, 0, 4],
    [4, 0, 0, 4],
    [4, 0, 0, 4],
    [4, 0, 0, 4],
    [4, 0, 0, 4],
    [4, 4, 4, 0]
  ],
  E: [
    [5, 5, 5, 5],
    [5, 0, 0, 0],
    [5, 0, 0, 0],
    [5, 5, 5, 0],
    [5, 0, 0, 0],
    [5, 0, 0, 0],
    [5, 5, 5, 5]
  ],
  I: [
    [9, 9, 9],
    [0, 9, 0],
    [0, 9, 0],
    [0, 9, 0],
    [0, 9, 0],
    [0, 9, 0],
    [9, 9, 9]
  ],
  J: [
    [10, 10, 10, 10],
    [0, 0, 0, 10],
    [0, 0, 0, 10],
    [0, 0, 0, 10],
    [10, 0, 0, 10],
    [10, 0, 0, 10],
    [10, 10, 10, 10]
  ],
  N: [
    [14, 0, 0, 0, 14],
    [14, 14, 0, 0, 14],
    [14, 14, 14, 0, 14],
    [14, 0, 14, 0, 14],
    [14, 0, 14, 14, 14],
    [14, 0, 0, 14, 14],
    [14, 0, 0, 0, 14]
  ],
  O: [
    [0, 15, 15, 0],
    [15, 0, 0, 15],
    [15, 0, 0, 15],
    [15, 0, 0, 15],
    [15, 0, 0, 15],
    [15, 0, 0, 15],
    [0, 15, 15, 0]
  ],
  S: [
    [19, 19, 19, 19],
    [19, 0, 0, 19],
    [19, 0, 0, 0],
    [19, 19, 19, 19],
    [0, 0, 0, 19],
    [19, 0, 0, 19],
    [19, 19, 19, 19]
  ],
  R: [
    [18, 18, 18, 18, 0],
    [18, 0, 0, 0, 18],
    [18, 0, 0, 0, 18],
    [18, 18, 18, 18, 0],
    [18, 0, 18, 0, 0],
    [18, 0, 18, 0, 0],
    [18, 0, 0, 18, 0]
  ],
  _dot: [
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [27, 27, 0],
    [27, 27, 0]
  ]
};

const charSchemaProsObj = {
  A: 1,
  B: 2,
  D: 4,
  E: 5,
  I: 9,
  J: 10,
  N: 14,
  O: 15,
  R: 18,
  S: 19,
  _dot: 27
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
