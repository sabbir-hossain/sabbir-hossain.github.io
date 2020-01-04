const inputText = [
  [
    "S",
    "e",
    "d",
    " ",
    "u",
    "t",
    " ",
    "p",
    "e",
    "r",
    "s",
    "p",
    "i",
    "c",
    "i",
    "a",
    "t",
    "i",
    "s",
    " ",
    "u",
    "n",
    "d",
    "e"
  ],
  [
    "o",
    "m",
    "n",
    "i",
    "s",
    " ",
    "i",
    "s",
    "t",
    "e",
    " ",
    "n",
    "a",
    "t",
    "u",
    "s",
    " ",
    "e",
    "r",
    "r",
    "o",
    "r",
    " ",
    " "
  ],
  [
    "s",
    "i",
    "t",
    " ",
    "v",
    "o",
    "l",
    "u",
    "p",
    "t",
    "a",
    "t",
    "e",
    "m",
    " ",
    "a",
    "c",
    "c",
    "u",
    "s",
    "a",
    "n",
    "t",
    "-"
  ],
  [
    "i",
    "u",
    "m",
    " ",
    "d",
    "o",
    "l",
    "o",
    "r",
    "e",
    "m",
    "q",
    "u",
    "e",
    " ",
    "d",
    "o",
    "l",
    "o",
    "r",
    " ",
    "s",
    "i",
    "t"
  ],
  [
    "l",
    "a",
    "u",
    "d",
    "a",
    "n",
    "t",
    "i",
    "u",
    "m",
    ",",
    " ",
    "t",
    "o",
    "t",
    "a",
    "m",
    " ",
    "r",
    "e",
    "m",
    " ",
    " ",
    " "
  ],
  [
    "a",
    "p",
    "e",
    "r",
    "i",
    "a",
    "m",
    ",",
    " ",
    "e",
    "a",
    "q",
    "u",
    "e",
    " ",
    "i",
    "p",
    "s",
    "a",
    " ",
    "q",
    "u",
    "a",
    "e"
  ],
  [
    "a",
    "b",
    " ",
    "i",
    "l",
    "l",
    "o",
    " ",
    "i",
    "n",
    "v",
    "e",
    "n",
    "t",
    "o",
    "r",
    "e",
    " ",
    "v",
    "e",
    "r",
    "i",
    "t",
    "-"
  ],
  [
    "a",
    "t",
    "i",
    "s",
    " ",
    "e",
    "t",
    " ",
    "q",
    "u",
    "a",
    "s",
    "i",
    " ",
    "a",
    "r",
    "c",
    "h",
    " ",
    "i",
    "t",
    "e",
    "c",
    "-"
  ],
  [
    "t",
    "o",
    " ",
    "b",
    "e",
    "a",
    "t",
    "a",
    "e",
    " ",
    "v",
    "i",
    "t",
    "a",
    "e",
    " ",
    "d",
    "i",
    "c",
    "t",
    " ",
    " ",
    " ",
    " "
  ]
];

// get a reference to the canvas
const canvas = document.getElementById("myCanvas");

// get a reference to the drawing context
const ctx = canvas.getContext("2d");

const cW = ctx.canvas.width;
const cH = ctx.canvas.height;

let startY = 50;
let startX = 50;

const maxLine = inputText.length;
const maxChar = inputText[0].length;

const displayRatio = 0.9;

const charList = ["J", "S"];

const colorList = ["orange", "green", "red", "blue", "indigo"];
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
console.log({ colorData, result });

// drawTextBG(ctx, txt, font, x, y, color)
startY = 50;
const font = 30;
const line = 30;
const minWidth = 27;

for (let i = 0, len = inputText.length; i < len; i++) {
  startX = 50;
  for (let j = 0, len2 = inputText[i].length; j < len2; j++) {
    let color = "#222";
    if (typeof colorData[result[i][j]] !== "undefined") {
      color = colorData[result[i][j]];
    }

    drawTextBG(ctx, inputText[i][j], font, startX, startY, color);
    startX += minWidth + 5;
  }
  startY += line + 7;
}
