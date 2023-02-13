const ASCII_SYMBOL_START = 33;
const ASCII_SYMBOL_END = 47;
const ASCII_NUMBER_START = 48;
const ASCII_NUMBER_END = 57;
const ASCII_UPPERCASE_START = 65;
const ASCII_UPPERCASE_END = 90;
const ASCII_LOWERCASE_START = 97;
const ASCII_LOWERCASE_END = 122;

enum CharacterType {
  LowerCaseLetter,
  UpperCaseLetter,
  Number,
  Symbol,
}

class CharacterRange {
  start: number;
  end: number;
  characterType: CharacterType;

  constructor(start: number, end: number, characterType: CharacterType) {
    this.start = start;
    this.end = end;
    this.characterType = characterType;
  }

  *[Symbol.iterator](): Iterator<string> {
    for (let i = this.start; i <= this.end; i++) {
      yield String.fromCharCode(i);
    }
  }
}

class PasswordGenerator {
  characterRanges: CharacterRange[];

  constructor() {
    this.characterRanges = [
      new CharacterRange(
        ASCII_LOWERCASE_START,
        ASCII_LOWERCASE_END,
        CharacterType.LowerCaseLetter
      ),
      new CharacterRange(
        ASCII_UPPERCASE_START,
        ASCII_UPPERCASE_END,
        CharacterType.UpperCaseLetter
      ),
      new CharacterRange(
        ASCII_NUMBER_START,
        ASCII_NUMBER_END,
        CharacterType.Number
      ),
      new CharacterRange(
        ASCII_SYMBOL_START,
        ASCII_SYMBOL_END,
        CharacterType.Symbol
      ),
    ];
  }

  generatePassword(
    nrLowerLetters: number,
    nrUpperLetters: number,
    nrSymbols: number,
    nrNumbers: number
  ) {
    let password: string[] = [];

    for (const range of this.characterRanges) {
      const chars = [...range];

      switch (range.characterType) {
        case CharacterType.LowerCaseLetter:
          for (let i = 0; i < nrLowerLetters; i++) {
            password.push(chars[Math.floor(Math.random() * chars.length)]);
          }
          break;

        case CharacterType.UpperCaseLetter:
          for (let i = 0; i < nrUpperLetters; i++) {
            password.push(chars[Math.floor(Math.random() * chars.length)]);
          }
          break;

        case CharacterType.Symbol:
          for (let i = 0; i < nrSymbols; i++) {
            password.push(chars[Math.floor(Math.random() * chars.length)]);
          }
          break;

        case CharacterType.Number:
          for (let i = 0; i < nrNumbers; i++) {
            password.push(chars[Math.floor(Math.random() * chars.length)]);
          }
          break;
      }
    }

    password.sort(() => Math.random() - 0.5);

    return password.join('');
  }
}

const passwordGenerator = new PasswordGenerator();
const password = passwordGenerator.generatePassword(2, 2, 2, 2);
console.log(password);
