define(["util"], function (util) {
  const TITLE_WIDTH = 90;

  const VOTES_SCHEMA = {
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

  const DIFFERENCE_SCHEMA = {
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
        scale: { domain: ["negative", "positive", "zero"] },
        legend: null,
      },
    },
  };

  const POSITIVE_SCHEMA = {
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

  const NEGATIVE_SCHEMA = {
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

  const configForSchema = function (schema, titleText, values) {
    schema.title.text = titleText;
    schema.data.values = values;

    return schema;
  };

  return {
    votes(titleText, values) {
      const title = util.splitTitle(titleText, TITLE_WIDTH);
      const data = util.toPositiveNegative(values);

      return configForSchema(VOTES_SCHEMA, title, data);
    },

    difference(titleText, values) {
      const title = util.splitTitle(
        titleText + " (разница голосов 'За' и 'Против')",
        TITLE_WIDTH,
      );
      const data = util.toDifference(values);

      return configForSchema(DIFFERENCE_SCHEMA, title, data);
    },

    positive(titleText, values) {
      const title = util.splitTitle(
        titleText + " (изменение числа голосов 'За')",
        TITLE_WIDTH,
      );
      const data = util.toPositiveDeltas(values);

      return configForSchema(POSITIVE_SCHEMA, title, data);
    },

    negative(titleText, values) {
      const title = util.splitTitle(
        titleText + " (изменение числа голосов 'Против')",
        TITLE_WIDTH,
      );
      const data = util.toNegativeDeltas(values);

      return configForSchema(NEGATIVE_SCHEMA, title, data);
    },
  };
});
