//https://github.com/missive/emoji-mart?tab=readme-ov-file#attributes--props

export type EmojiMartOptions = {
  skin: string;
  size: string;
  fallback: string;
  set: "twitter" | "native" | "apple" | "facebook" | "google";
  [x: string]: string;
};

