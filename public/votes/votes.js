define(["vue", "vega-embed", "config", "util"], function (
  vue,
  vega,
  config,
  util,
) {
  vue
    .createApp({
      mounted() {
        this.updateInitiatives();
      },

      data() {
        return {
          selected: null,
          initiatives: [],
          loading: false,
          showCharts: false,
          error: false,
          message: "",
        };
      },

      methods: {
        isSelected() {
          return this.selected !== null;
        },

        updateInitiatives() {
          fetch("/api/initiatives/list", {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify({}),
          })
            .then((data) => data.json())
            .then((data) => {
              if (data.success) {
                this.initiatives = data.items;
                this.error = false;
              } else {
                this.error = true;
                this.message = data.message;
              }

              this.loading = false;
            })
            .catch((error) => {
              this.loading = false;
              this.error = true;
              this.message = error;
            });

          this.loading = true;
        },

        showVotes() {
          const list = fetch("/api/votes/list", {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify({ id: this.selected.id }),
          }).then((data) => data.json());
          const hourly = fetch("api/votes/hourly", {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify({ id: this.selected.id, hours: 720 }),
          }).then((data) => data.json());

          Promise.all([list, hourly])
            .then((data) => {
              const list = data[0];
              const hourly = data[1];

              if (list.success && hourly.success) {
                const title = this.selected.name;
                const votes = util.toPositiveNegative(list.items);
                const difference = util.toDifference(list.items);
                const positive = util.toPositiveDeltas(hourly.items);
                const negative = util.toNegativeDeltas(hourly.items);

                vega("#votes-chart", config.votes(title, votes));
                vega("#difference-chart", config.difference(title, difference));
                vega("#positive-chart", config.positive(title, positive));
                vega("#negative-chart", config.negative(title, negative));

                this.showCharts = true;
                this.error = false;
              } else {
                this.error = true;
                this.message = data.message;
              }

              this.loading = false;
            })
            .catch((error) => {
              this.loading = false;
              this.error = true;
              this.message = error;
            });

          this.loading = true;
        },
      },
    })
    .mount("#app");
});
