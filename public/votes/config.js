define(function () {
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
      return configForSchema(VOTES_SCHEMA, titleText, values);
    },

    difference(titleText, values) {
      return configForSchema(DIFFERENCE_SCHEMA, titleText, values);
    },

    positive(titleText, values) {
      return configForSchema(POSITIVE_SCHEMA, titleText, values);
    },

    negative(titleText, values) {
      return configForSchema(NEGATIVE_SCHEMA, titleText, values);
    },
  };
});
