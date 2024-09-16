// @ts-nocheck
import { LetterObjDto } from "@/models/dto/letterObj-dto";


// this is just library its not grid for input text
export class LettersLibObjDto {
  letters: Record<string, LetterObjDto> | {};

  constructor(data: LettersLibObjDto) {
    this.letters = data.letters;

    Object.preventExtensions(this as Object);
  }
}
