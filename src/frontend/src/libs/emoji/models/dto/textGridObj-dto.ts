// @ts-nocheck
import { Resolution } from "@/types/Grid";

interface TextGridObj {
  text: string;
}

export class TextGridObjDto {
  #layout: any[][] | null;
  #resolution: Resolution | null;
  #text: string;

  constructor({ text }: TextGridObj | null) {
    this.#text = text;

    Object.preventExtensions(this as Object);
  }

  get layout() {
    return this.#layout;
  }

  get resolution() {
    return this.#resolution;
  }
}
