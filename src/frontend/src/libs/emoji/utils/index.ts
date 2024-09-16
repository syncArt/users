//fill row
export const fr = (amount: number, ascii: string) => {
  return Array.from({ length: amount }, () => ascii);
};
