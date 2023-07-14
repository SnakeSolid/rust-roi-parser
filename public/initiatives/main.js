requirejs.config({
  waitSeconds: 15,
  baseUrl: "/initiatives",
  paths: {
    initiatives: "initiatives",
    vue: "https://cdnjs.cloudflare.com/ajax/libs/vue/3.3.4/vue.global.min",
  },
  shim: {
    vue: {
      exports: "Vue",
    },
  },
});

requirejs(
  ["initiatives"],
  (initiatives) => {},
  (error) => console.log(error.message),
);
