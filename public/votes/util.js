define(function () {
  return {
    hourlyData(items, length) {
      const hours = new Set();
      const result = [];

      for (let index = items.length - 1; index >= 0; index -= 1) {
        const row = items[index];
        const hour = row.datetime.substring(0, 13) + ":00:00";

        if (!hours.has(hour)) {
          result.push({
            datetime: hour,
            positive: row.positive,
            negative: row.negative,
          });

          if (result.length >= length) {
            break;
          }

          hours.add(hour);
        }
      }

      result.reverse();

      return result;
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
