define(function () {
  return {
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
