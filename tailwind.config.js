/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    './src/**/*.{html,rs}'
  ],
  theme: {
    extend: {
      fontFamily: {
        sans: ['Inter'],
        mono: ['JetBrains Mono'],
      },
    },
  },
  plugins: [],
}

