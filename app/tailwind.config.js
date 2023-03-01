module.exports = {
  content: ["./src/**/*.{rs, html, css}", "./index.html"],
  theme: {
    extend: {},
  },
  plugins: [require("@tailwindcss/typography")],
};
