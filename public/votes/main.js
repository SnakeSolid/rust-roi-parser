requirejs.config({
  waitSeconds: 15,
  baseUrl: "/votes",
  paths: {
    config: "config",
    util: "util",
    votes: "votes",
    vue: "https://cdnjs.cloudflare.com/ajax/libs/vue/3.3.4/vue.global.min",
    vega: "https://cdnjs.cloudflare.com/ajax/libs/vega/5.25.0/vega.min",
    "vega-lite":
      "https://cdnjs.cloudflare.com/ajax/libs/vega-lite/5.13.0/vega-lite.min",
    "vega-embed":
      "https://cdnjs.cloudflare.com/ajax/libs/vega-embed/6.22.1/vega-embed.min",
  },
  shim: {
    "vega-embed": {
      deps: ["vega-lite", "vega"],
      exports: "vegaEmbed",
    },
    vue: {
      exports: "Vue",
    },
  },
});

requirejs(
  ["votes"],
  (votes) => {},
  (error) => console.log(error.message),
);
