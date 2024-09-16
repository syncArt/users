const colors = require("tailwindcss/colors");

/** @type {import("tailwindcss").Config} */
module.exports = {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx,mdx}",
    "./node_modules/flowbite/**/*.js"
  ],
  theme: {
    colors: {
      ...colors,
      primary: "#2EF867",
      secondary: "#FEF141",
      white: "#FFFDFD",
      black: "#000000",
      pink: "#FE26A6",
      orange: "#FEB241",
      blue: "#00B3FF",
      grey: "#232323"
    },
    extend: {
      fontFamily: {
        sequel100Black: ["\"Sequel100Black\"", "sans-serif"],
        spaceMono: ["\"SpaceMono\"", "monospace"]
      },
      fontWeight: {
        45: "45",
        55: "55",
        65: "65",
        75: "75",
        85: "85",
        95: "95",
        105: "105",
        115: "115",
        regular: "400",
        bold: "700"
      },
      borderRadius: {
        medium: "22px"
      }
    },
    screens: {
      tablet: "640px",
      laptop: "1024px",
      desktop: "1280px"
    }
  }
};
