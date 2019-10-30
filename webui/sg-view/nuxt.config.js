const path = require("path");

module.exports = {
  mode: "universal",
  plugins: [
    "~plugins/vuetify.js",
    "~plugins/formatdate.js",
    "~plugins/chartkick.js",
    "~plugins/truncate.js",
    "~plugins/numeral.js"
  ],

  /*
  ** Headers of the page
  */
  head: {
    title: "sg-view",
    meta: [
      {charset: "utf-8"},
      {name: "viewport", content: "width=device-width, initial-scale=1"},
      {hid: "description", name: "description", content: "SightGlass viewer"}
    ],
    link: [{rel: "icon", type: "image/x-icon", href: "/favicon.ico"}]
  },

  css: [
    {
      src: path.join(__dirname, "assets/css/app.styl"),
      lang: "styl"
    }
  ],

  /*
  ** Customize the progress bar color
  */
  loading: {color: "#3B8070"},
  /*
  ** Build configuration
  */
  build: {
    vendor: ["~/plugins/vuetify.js"],
    /*
    ** Run ESLint on save
    */
    extend(config, {isDev, isClient}) {
      if (isDev && isClient) {
        config.module.rules.push({
          enforce: "pre",
          test: /\.(js|vue)$/,
          loader: "eslint-loader",
          exclude: /(node_modules)/
        });
      }
    }
  },

  /*
  ** Pass the HISTORY_URL from the server environment to the client for accessing different sg-history servers; though
  ** this line may seem redundant, it ensures that the client receives the server-defined variable.
   */
  env: {
    HISTORY_URL: process.env.HISTORY_URL || 'http://localhost:8001',
  }
};
