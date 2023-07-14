define(["vue"], function (vue) {
  const URL_REGEX = new RegExp("^https:\\/\\/www\\.roi\\.ru\\/(\\d+)\\/?$");

  vue
    .createApp({
      mounted() {
        this.updateInitiatives();
      },

      data() {
        return {
          url: "",
          items: [],
          loading: false,
          error: false,
          message: "",
        };
      },

      methods: {
        isUrlError() {
          return this.url !== "" && !URL_REGEX.test(this.url);
        },

        isUrlValid() {
          return URL_REGEX.test(this.url);
        },

        hasItems() {
          return this.items.length > 0;
        },

        initiativeLink(item) {
          return `https://www.roi.ru/${item.id}/`;
        },

        addInitiative() {
          const result = this.url.match(URL_REGEX);

          if (result === null) {
            return;
          }

          fetch("/api/initiatives/add", {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify({ id: parseInt(result[1]) }),
          })
            .then((data) => data.json())
            .then((data) => {
              if (!data.success) {
                this.error = true;
                this.message = data.message;
              }

              this.updateInitiatives();
            })
            .catch((error) => {
              this.loading = false;
              this.error = true;
              this.message = error;
            });

          this.loading = true;
        },

        enableInitiative(item) {
          fetch("/api/initiatives/enable", {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify({ id: item.id }),
          })
            .then((data) => data.json())
            .then((data) => {
              if (!data.success) {
                this.error = true;
                this.message = data.message;
              }

              this.updateInitiatives();
            })
            .catch((error) => {
              this.loading = false;
              this.error = true;
              this.message = error;
            });

          this.loading = true;
        },

        disableInitiative(item) {
          fetch("/api/initiatives/disable", {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify({ id: item.id }),
          })
            .then((data) => data.json())
            .then((data) => {
              if (!data.success) {
                this.error = true;
                this.message = data.message;
              }

              this.updateInitiatives();
            })
            .catch((error) => {
              this.loading = false;
              this.error = true;
              this.message = error;
            });

          this.loading = true;
        },

        removeInitiative(item) {
          if (
            !window.confirm(
              `Are you sure to remove all data for initiative "${item.name}"?`,
            )
          ) {
            return;
          }

          fetch("/api/initiatives/remove", {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify({ id: item.id }),
          })
            .then((data) => data.json())
            .then((data) => {
              if (!data.success) {
                this.error = true;
                this.message = data.message;
              }

              this.updateInitiatives();
            })
            .catch((error) => {
              this.loading = false;
              this.error = true;
              this.message = error;
            });

          this.loading = true;
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
                this.items = data.items;
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
