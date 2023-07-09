"use strict";

const votes_schema = {
  $schema: "https://vega.github.io/schema/vega-lite/v5.json",
  width: 1127,
  height: 600,
  autosize: { type: "fit", contains: "padding" },
  data: { values: [] },
  title: { text: [], fontSize: 16 },
  mark: { type: "line", strokeWidth: 3, strokeJoin: "round" },
  encoding: {
    x: {
      field: "datetime",
      type: "temporal",
      axis: {
        format: "%Y.%m.%d",
        labelFontSize: 12,
        labelAngle: -60,
        title: null,
      },
    },
    y: {
      field: "value",
      type: "quantitative",
      scale: { zero: false },
      axis: { format: ".0f", labelFontSize: 12, title: null },
    },
    color: {
      field: "type",
      type: "nominal",
      legend: {
        labelExpr: "datum.label === 'positive' ? 'За' : 'Против'",
        title: "Голоса",
        titleFontSize: 14,
        labelFontSize: 12,
      },
    },
  },
};

const difference_schema = {
  $schema: "https://vega.github.io/schema/vega-lite/v5.json",
  width: 1127,
  height: 600,
  autosize: { type: "fit", contains: "padding" },
  data: { values: [] },
  title: { text: [], fontSize: 16 },
  mark: { type: "line", strokeWidth: 3, strokeJoin: "round" },
  encoding: {
    x: {
      field: "datetime",
      type: "temporal",
      axis: {
        format: "%Y.%m.%d",
        labelFontSize: 12,
        labelAngle: -60,
        title: null,
      },
    },
    y: {
      field: "value",
      type: "quantitative",
      scale: { zero: false },
      axis: { format: ".0f", labelFontSize: 12, title: null },
    },
    color: {
      field: "type",
      type: "nominal",
      legend: null,
    },
  },
};

const positive_schema = {
  $schema: "https://vega.github.io/schema/vega-lite/v5.json",
  width: 1127,
  height: 600,
  autosize: { type: "fit", contains: "padding" },
  data: { values: [] },
  transform: [
    {
      window: [
        {
          field: "value",
          op: "average",
          as: "mean",
        },
      ],
      frame: [-6, 6],
    },
  ],
  title: { text: [], fontSize: 16 },
  encoding: {
    x: {
      field: "datetime",
      type: "temporal",
      axis: {
        format: "%Y.%m.%d",
        labelFontSize: 12,
        labelAngle: -60,
        title: null,
      },
    },
  },
  layer: [
    {
      mark: { type: "line", strokeWidth: 2, strokeJoin: "round" },
      encoding: {
        y: {
          field: "value",
          type: "quantitative",
          axis: { labelFontSize: 12, title: null },
        },
      },
    },
    {
      mark: {
        type: "line",
        color: "#f58518",
        strokeWidth: 3,
        strokeJoin: "round",
      },
      encoding: {
        y: {
          field: "mean",
          type: "quantitative",
          axis: { labelFontSize: 12, title: null },
        },
      },
    },
  ],
};

const negative_schema = {
  $schema: "https://vega.github.io/schema/vega-lite/v5.json",
  width: 1127,
  height: 600,
  autosize: { type: "fit", contains: "padding" },
  data: { values: [] },
  transform: [
    {
      window: [
        {
          field: "value",
          op: "average",
          as: "mean",
        },
      ],
      frame: [-6, 6],
    },
  ],
  title: { text: [], fontSize: 16 },
  encoding: {
    x: {
      field: "datetime",
      type: "temporal",
      axis: {
        format: "%Y.%m.%d",
        labelFontSize: 12,
        labelAngle: -60,
        title: null,
      },
    },
  },
  layer: [
    {
      mark: { type: "line", strokeWidth: 2, strokeJoin: "round" },
      encoding: {
        y: {
          field: "value",
          type: "quantitative",
          axis: { labelFontSize: 12, title: null },
        },
      },
    },
    {
      mark: {
        type: "line",
        color: "#f58518",
        strokeWidth: 3,
        strokeJoin: "round",
      },
      encoding: {
        y: {
          field: "mean",
          type: "quantitative",
          axis: { labelFontSize: 12, title: null },
        },
      },
    },
  ],
};
const application = Vue.createApp({
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
      fetch("/api/votes/list", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ id: this.selected.id }),
      })
        .then((data) => data.json())
        .then((data) => {
          if (data.success) {
            const votes = data.items
              .map((row) => {
                return {
                  datetime: row.datetime,
                  type: "positive",
                  value: row.positive,
                };
              })
              .concat(
                data.items.map((row) => {
                  return {
                    datetime: row.datetime,
                    type: "negative",
                    value: row.negative,
                  };
                }),
              );
            const difference = data.items
              .map((row) => {
                return {
                  datetime: row.datetime,
                  type: row.positive >= row.negative ? "positive" : "negative",
                  value: row.positive - row.negative,
                };
              })
              .concat([
                {
                  datetime: data.items.reduce(
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
                  datetime: data.items.reduce(
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
            const positive = data.items
              .filter((row, index, data) => index > data.length - 30 * 24)
              .map((row, index, data) => {
                return {
                  datetime: row.datetime,
                  value:
                    index > 0 ? row.positive - data[index - 1].positive : 0,
                };
              });
            const negative = data.items
              .filter((row, index, data) => index > data.length - 30 * 24)
              .map((row, index, data) => {
                return {
                  datetime: row.datetime,
                  value:
                    index > 0 ? row.negative - data[index - 1].negative : 0,
                };
              });

            votes_schema.title.text = this.splitTitle(this.selected.name, 90);
            votes_schema.data.values = votes;
            vegaEmbed("#votes-chart", votes_schema);

            difference_schema.title.text = this.splitTitle(
              this.selected.name + " (разница голосов 'За' и 'Против')",
              90,
            );
            difference_schema.data.values = difference;
            vegaEmbed("#difference-chart", difference_schema);

            positive_schema.title.text = this.splitTitle(
              this.selected.name + " (изменение числа голосов 'За')",
              90,
            );
            positive_schema.data.values = positive;
            vegaEmbed("#positive-chart", positive_schema);

            negative_schema.title.text = this.splitTitle(
              this.selected.name + " (изменение числа голосов 'Против')",
              90,
            );
            negative_schema.data.values = negative;
            vegaEmbed("#negative-chart", negative_schema);

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

    splitTitle(text, length) {
      if (text.length <= length) {
        return [text];
      }

      const result = [];
      let tail = text;

      while (tail !== "") {
        let nextIndex = tail.search(",");

        if (nextIndex !== -1) {
          result.push(tail.substring(0, nextIndex + 1).trim());
          tail = tail.slice(nextIndex + 1).trim();

          continue;
        }

        nextIndex = tail.slice(length).search(" ");

        if (nextIndex !== -1) {
          result.push(tail.substring(0, length + nextIndex).trim());
          tail = tail.slice(length + nextIndex).trim();
        } else {
          result.push(tail);

          break;
        }
      }

      return result;
    },
  },
});
application.mount("#app");
