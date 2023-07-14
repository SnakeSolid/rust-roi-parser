define(function () {
  const minDatetime = function (items) {
    return items.reduce(
      (accumulator, row) =>
        accumulator === null || accumulator > row.datetime
          ? row.datetime
          : accumulator,
      null,
    );
  };

  const maxDatetime = function (items) {
    return items.reduce(
      (accumulator, row) =>
        accumulator === null || accumulator < row.datetime
          ? row.datetime
          : accumulator,
      null,
    );
  };

  return {
    maxPositive(items) {
      return items.reduce(
        (accumulator, row) =>
          accumulator < row.positive ? row.positive : accumulator,
        0,
      );
    },

    maxNegative(items) {
      return items.reduce(
        (accumulator, row) =>
          accumulator < row.negative ? row.negative : accumulator,
        0,
      );
    },

    toPositiveNegative(items) {
      return items
        .map((row) => {
          return {
            datetime: row.datetime,
            type: "positive",
            value: row.positive,
          };
        })
        .concat(
          items.map((row) => {
            return {
              datetime: row.datetime,
              type: "negative",
              value: row.negative,
            };
          }),
        );
    },

    toDifference(items) {
      return items
        .map((row) => {
          return {
            datetime: row.datetime,
            type: row.positive >= row.negative ? "positive" : "negative",
            value: row.positive - row.negative,
          };
        })
        .concat([
          {
            datetime: minDatetime(items),
            type: "zero",
            value: 0,
          },
          {
            datetime: maxDatetime(items),
            type: "zero",
            value: 0,
          },
        ]);
    },

    toPositiveDeltas(items) {
      return items.map((row, index, data) => {
        return {
          datetime: row.datetime,
          value: index > 0 ? row.positive - data[index - 1].positive : 0,
        };
      });
    },

    toNegativeDeltas(items) {
      return items.map((row, index, data) => {
        return {
          datetime: row.datetime,
          value: index > 0 ? row.negative - data[index - 1].negative : 0,
        };
      });
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
  };
});
