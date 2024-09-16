//@ts-nocheck

export const fr = (amount: number, ascii: string) => {
  return Array.from({ length: amount }, () => ascii);
};

const PixelMojis_builder_function = {
  "1": (x, y, z) => [
    [fr(5, z), fr(5, y), fr(5, z)],
    [fr(3, z), fr(2, y), fr(5, x), fr(2, y), fr(3, z)],
    [fr(2, z), fr(1, y), fr(9, x), fr(1, y), fr(2, z)],
    [fr(1, z), fr(1, y), fr(11, x), fr(1, y), fr(1, z)],
    [
      fr(1, z),
      fr(1, y),
      fr(3, x),
      fr(1, y),
      fr(3, x),
      fr(1, y),
      fr(3, x),
      fr(1, y),
      fr(1, z)
    ],
    [fr(1, y), fr(4, x), fr(1, y), fr(3, x), fr(1, y), fr(4, x), fr(1, y)],
    [fr(1, y), fr(4, x), fr(1, y), fr(3, x), fr(1, y), fr(4, x), fr(1, y)],
    [fr(1, y), fr(13, x), fr(1, y)],
    [fr(1, y), fr(2, x), fr(1, y), fr(7, x), fr(1, y), fr(2, x), fr(1, y)],
    [fr(1, y), fr(2, x), fr(1, y), fr(7, x), fr(1, y), fr(2, x), fr(1, y)],
    [
      fr(1, z),
      fr(1, y),
      fr(2, x),
      fr(1, y),
      fr(5, x),
      fr(1, y),
      fr(2, x),
      fr(1, y),
      fr(1, z)
    ],
    [fr(1, z), fr(1, y), fr(3, x), fr(5, y), fr(3, x), fr(1, y), fr(1, z)],
    [fr(2, z), fr(1, y), fr(9, x), fr(1, y), fr(2, z)],
    [fr(3, z), fr(2, y), fr(5, x), fr(2, y), fr(3, z)],
    [fr(5, z), fr(5, y), fr(5, z)]
  ],
  "2": (x, y, z) => [
    [fr(5, z), fr(5, y), fr(5, z)],
    [fr(3, z), fr(2, y), fr(5, x), fr(2, y), fr(3, z)],
    [fr(2, z), fr(1, y), fr(9, x), fr(1, y), fr(2, z)],
    [
      fr(1, z),
      fr(1, y),
      fr(2, x),
      fr(1, y),
      fr(5, x),
      fr(1, y),
      fr(2, x),
      fr(1, y),
      fr(1, z)
    ],
    [
      fr(1, z),
      fr(1, y),
      fr(3, x),
      fr(1, y),
      fr(3, x),
      fr(1, y),
      fr(3, x),
      fr(1, y),
      fr(1, z)
    ],
    [fr(2, y), fr(1, x), fr(4, y), fr(1, x), fr(4, y), fr(1, x), fr(2, y)],
    [fr(1, y), fr(13, x), fr(1, y)],
    [fr(1, y), fr(1, x), fr(11, y), fr(1, x), fr(1, y)],
    [fr(1, y), fr(1, x), fr(1, y), fr(9, z), fr(1, y), fr(1, x), fr(1, y)],
    [fr(1, y), fr(1, x), fr(11, y), fr(1, x), fr(1, y)],
    [fr(1, z), fr(1, y), fr(1, x), fr(9, y), fr(1, x), fr(1, y), fr(1, z)],
    [
      fr(1, z),
      fr(1, y),
      fr(2, x),
      fr(1, y),
      fr(5, z),
      fr(1, y),
      fr(2, x),
      fr(1, y),
      fr(1, z)
    ],
    [fr(2, z), fr(1, y), fr(2, x), fr(5, y), fr(2, x), fr(1, y), fr(2, z)],
    [fr(3, z), fr(2, y), fr(5, x), fr(2, y), fr(3, z)],
    [fr(5, z), fr(5, y), fr(5, z)]
  ],
  "3": (x, y, z) => [
    [fr(1, z), fr(1, y), fr(11, z), fr(1, y), fr(1, z)],
    [
      fr(1, y),
      fr(1, x),
      fr(1, y),
      fr(2, z),
      fr(5, y),
      fr(2, z),
      fr(1, y),
      fr(1, x),
      fr(1, y)
    ],
    [fr(1, y), fr(2, x), fr(2, y), fr(5, x), fr(2, y), fr(2, x), fr(1, y)],
    [fr(1, y), fr(1, x), fr(1, y), fr(9, x), fr(1, y), fr(1, x), fr(1, y)],
    [fr(1, z), fr(1, y), fr(11, x), fr(1, y), fr(1, z)],
    [fr(1, z), fr(1, y), fr(11, x), fr(1, y), fr(1, z)],
    [fr(1, y), fr(13, x), fr(1, y)],
    [fr(1, y), fr(13, x), fr(1, y)],
    [fr(1, y), fr(2, x), fr(2, y), fr(5, x), fr(2, y), fr(2, x), fr(1, y)],
    [
      fr(1, y),
      fr(3, x),
      fr(1, z),
      fr(1, y),
      fr(3, x),
      fr(1, y),
      fr(1, z),
      fr(3, x),
      fr(1, y)
    ],
    [fr(1, z), fr(1, y), fr(11, x), fr(1, y), fr(1, z)],
    [
      fr(1, z),
      fr(1, y),
      fr(3, x),
      fr(1, y),
      fr(3, x),
      fr(1, y),
      fr(3, x),
      fr(1, y),
      fr(1, z)
    ],
    [fr(2, z), fr(1, y), fr(3, x), fr(3, y), fr(3, x), fr(1, y), fr(2, z)],
    [fr(3, z), fr(2, y), fr(5, x), fr(2, y), fr(3, z)],
    [fr(5, z), fr(5, y), fr(5, z)]
  ],
  " ": (x, y) => [[y], [y], [y], [y]]
};

export default PixelMojis_builder_function;
