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
      colors: {
        'sidebar': '#1c1917', // stone-900
        'primary': {
          light: '#d6d3d1', // stone-300
          DEFAULT: '#78716c', // stone-500
          dark: '#44403c',   // stone-700
        },
        'secondary': {
          light: '#e7e5e4', // stone-200
          DEFAULT: '#a8a29e', // stone-400
          dark: '#57534e',   // stone-600
        }
      },
      spacing: {
        'sidebar': '80px',
      }
    },
  },
  plugins: [],
}

