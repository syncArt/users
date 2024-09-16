//@ts-nocheck

export const fr = (amount: number, ascii: string) => {
  return Array.from({ length: amount }, () => ascii);
};

const ASCII_3x4_letters_builder_function = {
  a: (x, y) => [[y, ...fr(2, x)], [x, y, x], fr(3, x), [x, y, x]],
  b: (x, y) => [[x, ...fr(2, y)], fr(3, x), [x, y, x], fr(3, x)],
  c: (x, y) => [fr(3, x), [x, ...fr(2, y)], [x, ...fr(2, y)], fr(3, x)],
  d: (x, y) => [
    [...fr(2, x), y],
    [x, y, x],
    [x, y, x],
    [...fr(2, x), y]
  ],
  e: (x, y) => [fr(3, x), [x, ...fr(2, y)], [...fr(2, x), y], fr(3, x)],
  f: (x, y) => [fr(3, x), [x, ...fr(2, y)], [...fr(2, x), y], [x, ...fr(2, y)]],
  g: (x, y) => [fr(3, x), [x, ...fr(2, y)], [x, y, x], fr(3, x)],
  h: (x, y) => [[x, y, x], [x, y, x], fr(3, x), [x, y, x]],
  i: (x, y) => [[x], [x], [x], [x]],
  j: (x, y) => [[...fr(2, y), x], [...fr(2, y), x], [x, y, x], fr(3, x)],
  k: (x, y) => [
    [x, y, x],
    [...fr(2, x), y],
    [x, y, x],
    [x, y, x]
  ],
  l: (x, y) => [[x, ...fr(2, y)], [x, ...fr(2, y)], [x, ...fr(2, y)], fr(3, x)],
  m: (x, y) => [
    [...fr(2, x), y, ...fr(2, x)],
    [x, y, x, y, x],
    [x, y, x, y, x],
    [x, ...fr(3, y), x]
  ],
  n: (x, y) => [
    [x, ...fr(2, y), x],
    [...fr(2, x), y, x],
    [x, y, ...fr(2, x)],
    [x, ...fr(2, y), x]
  ],
  o: (x, y) => [fr(3, x), [x, y, x], [x, y, x], fr(3, x)],
  p: (x, y) => [fr(3, x), [x, y, x], fr(3, x), [x, ...fr(2, y)]],
  r: (x, y) => [fr(3, x), [x, y, x], [...fr(2, x), y], [x, y, x]],
  s: (x, y) => [fr(2, x), [x, y], [y, x], fr(2, x)],
  t: (x, y) => [fr(3, x), [y, x, y], [y, x, y], [y, x, y]],
  u: (x, y) => [[x, y, x], [x, y, x], [x, y, x], fr(3, x)],
  w: (x, y) => [
    [x, y, x, y, x],
    [x, y, x, y, x],
    [x, y, x, y, x],
    [y, x, y, x, y]
  ],
  z: (x, y) => [fr(2, x), [y, x], [x, y], fr(2, x)],
  x: (x, y) => [
    [x, y, x],
    [y, x, y],
    [x, y, x],
    [x, y, x]
  ],
  y: (x, y) => [[x, y, x], fr(3, x), [y, x, y], [y, x, y]],
  q: (x, y) => [fr(3, x), [x, y, x], fr(3, x), [...fr(2, y), x]],
  v: (x, y) => [
    [x, y, x],
    [x, y, x],
    [x, y, x],
    [y, x, y]
  ],
  "0": (x, y) => [
    [x, x, x],
    [x, y, x],
    [x, y, x],
    [x, x, x]
  ],
  "1": (x, y) => [
    [x, x, y],
    [y, x, y],
    [y, x, y],
    [x, x, x]
  ],
  "2": (x, y) => [
    [x, x, x],
    [y, y, x],
    [x, x, y],
    [x, x, x]
  ],
  "3": (x, y) => [
    [x, x, x],
    [y, x, y],
    [y, y, x],
    [x, x, x]
  ],
  "4": (x, y) => [
    [x, y, x],
    [x, y, x],
    [x, x, x],
    [y, y, x]
  ],
  "5": (x, y) => [
    [x, x, x],
    [x, y, y],
    [y, x, x],
    [x, x, x]
  ],
  "6": (x, y) => [
    [x, x, x],
    [x, y, y],
    [x, x, x],
    [x, x, x]
  ],
  "7": (x, y) => [
    [x, x, x],
    [y, y, x],
    [y, x, y],
    [y, x, y]
  ],
  "8": (x, y) => [
    [x, x, x],
    [x, y, x],
    [x, x, x],
    [x, x, x]
  ],
  "9": (x, y) => [
    [x, x, x],
    [x, y, x],
    [x, x, x],
    [y, y, x]
  ],
  ":": (x, y) => [[y], [x], [y], [x]],
  "!": (x, y) => [[x], [x], [y], [x]],
  ")": (x, y) => [
    [x, y],
    [y, x],
    [y, x],
    [x, y]
  ],
  "(": (x, y) => [
    [y, x],
    [x, y],
    [x, y],
    [y, x]
  ],
  ".": (x, y) => [[y], [y], [y], [x]],
  "?": (x, y) => [
    [x, x, x, x],
    [y, y, y, x],
    [y, x, x, y],
    [y, x, y, y]
  ],
  " ": (x, y) => [[y], [y], [y], [y]]
};

export default ASCII_3x4_letters_builder_function;
