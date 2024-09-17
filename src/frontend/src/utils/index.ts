export function optionalCandidValueToValue<T>(optVal: [T] | []): T | undefined {
  return optVal.length > 0 ? optVal[0] : undefined;
}

export function valueToOptionalCandidValue<T>(val: T | undefined): [T] | [] {
  return val !== undefined ? [val] : [];
}
