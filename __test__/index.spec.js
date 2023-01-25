const test = require('ava');
const fs = require('fs');
const path = require('path');

const { readImage, writeImage } = require('../index.js');

test('write and read PNG data', (t) => {
  const buffer = fs.readFileSync(path.join(__dirname, 'data/test.png'));
  console.log(buffer);
  writeImage(buffer);
  const result = readImage();
  t.is(result.length, buffer.length);
})
