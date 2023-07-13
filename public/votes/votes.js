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
                const votes = list.items
                  .map((row) => {
                    return {
                      datetime: row.datetime,
                      type: "positive",
                      value: row.positive,
                    };
                  })
                  .concat(
                    list.items.map((row) => {
                      return {
                        datetime: row.datetime,
                        type: "negative",
                        value: row.negative,
                      };
                    }),
                  );
                const difference = list.items
                  .map((row) => {
                    return {
                      datetime: row.datetime,
                      type:
                        row.positive >= row.negative ? "positive" : "negative",
                      value: row.positive - row.negative,
                    };
                  })
                  .concat([
                    {
                      datetime: list.items.reduce(
                        (accumulator, row) =>
                          accumulator === null || accumulator > row.datetime
                            ? row.datetime
                            : accumulator,
                        null,
                      ),
                      type: "zero",
                      value: 0,
                    },
                    {
                      datetime: list.items.reduce(
                        (accumulator, row) =>
                          accumulator === null || accumulator < row.datetime
                            ? row.datetime
                            : accumulator,
                        null,
                      ),
                      type: "zero",
                      value: 0,
                    },
                  ]);
                const positive = hourly.items.map((row, index, data) => {
                  return {
                    datetime: row.datetime,
                    value:
                      index > 0 ? row.positive - data[index - 1].positive : 0,
                  };
                });
                const negative = hourly.items.map((row, index, data) => {
                  return {
                    datetime: row.datetime,
                    value:
                      index > 0 ? row.negative - data[index - 1].negative : 0,
                  };
                });

                vega(
                  "#votes-chart",
                  config.votes(util.splitTitle(this.selected.name, 90), votes),
                );
                vega(
                  "#difference-chart",
                  config.difference(
                    util.splitTitle(
                      this.selected.name + " (разница голосов 'За' и 'Против')",
                      90,
                    ),
                    difference,
                  ),
                );
                vega(
                  "#positive-chart",
                  config.positive(
                    util.splitTitle(
                      this.selected.name + " (изменение числа голосов 'За')",
                      90,
                    ),
                    positive,
                  ),
                );
                vega(
                  "#negative-chart",
                  config.negative(
                    util.splitTitle(
                      this.selected.name +
                        " (изменение числа голосов 'Против')",
                      90,
                    ),
                    negative,
                  ),
                );

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
