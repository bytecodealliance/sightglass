<template>
      <v-container>
      <v-flex>
<v-layout row wrap>
        <v-autocomplete label="Test:" :items="tests_names" v-model="test_name" menu-props="auto" clearable placeholder="Display only results from a specific test" />
        <v-autocomplete label="Branch:" :items="gitrefs" v-model="gitref" menu-props="auto" clearable placeholder="Display only results from a specific branch" />
        <v-flex sm4><v-switch label="Plot all tests" v-model="plot_all" /></v-flex>
</v-layout>
<v-layout row wrap>
  <v-flex sm4>
      <v-menu
        ref="menu_begin"
        lazy
        :close-on-content-click="false"
        v-model="menu_begin"
        transition="scale-transition"
        offset-y
        full-width
        :nudge-right="40"
        min-width="290px"
        :return-value.sync="date_begin"
      >
        <v-text-field
          slot="activator"
          label="Start date"
          v-model="date_begin"
          prepend-icon="event"
          readonly
        ></v-text-field>
        <v-date-picker v-model="date_begin" no-title scrollable>
          <v-spacer></v-spacer>
          <v-btn flat color="primary" @click="menu_begin = false">Cancel</v-btn>
          <v-btn flat color="primary" @click="$refs.menu_begin.save(date_begin)">OK</v-btn>
        </v-date-picker>
      </v-menu>
    </v-flex>

  <v-flex sm4>
      <v-menu
        ref="menu_end"
        lazy
        :close-on-content-click="false"
        v-model="menu_end"
        transition="scale-transition"
        offset-y
        full-width
        :nudge-right="40"
        min-width="290px"
        :return-value.sync="date_end"
      >
        <v-text-field
          slot="activator"
          label="End date"
          v-model="date_end"
          prepend-icon="event"
          readonly
        ></v-text-field>
        <v-date-picker v-model="date_end" no-title scrollable>
          <v-spacer></v-spacer>
          <v-btn flat color="primary" @click="menu_end = false">Cancel</v-btn>
          <v-btn flat color="primary" @click="$refs.menu_end.save(date_end)">OK</v-btn>
        </v-date-picker>
      </v-menu>
    </v-flex>

</v-layout>
      </v-flex>
      <line-chart :data="chartdata"></line-chart>
      <v-data-table v-bind:pagination.sync="pagination" :headers="headers" :items="filtered_items" :loading="loading" class="elevation-1" :rows-per-page-items='[10,25,50,{"text":"All","value":-1}]'>
          <template slot="items" slot-scope="props">
              <td>
                  {{ props.item.meta.ts | formatDate }}
              </td>
              <td>
                  {{ props.item.meta.author }}
              </td>
              <td>
                  {{ props.item.commit_id | truncate(16) }}
              </td>
              <td>
                  {{ props.item.meta.gitref | truncate(32) }}
              </td>
              <td>
                  {{ props.item.meta.message | truncate(100) }}
              </td>
              <td>
                  {{ props.item.perf | truncateNumber }}
              </td>
          </template>
      </v-data-table>
      </v-container>
</template>
<script>
import axios from "axios";
export default {
  props: ["loading", "items", "tests_names", "gitrefs"],
  computed: {
    filtered_items: function() {
      let items = this.items;
      this.gitref &&
        (items = items.filter(item => item.meta.gitref === this.gitref));
      let ts_begin = 0,
        ts_end = Number.MAX_SAFE_INTEGER;
      this.date_begin && (ts_begin = new Date(this.date_begin).getTime());
      this.date_end && (ts_end = new Date(this.date_end).getTime());
      if (ts_begin > 0 && ts_end > 0 && ts_begin < ts_end) {
        items = items.filter(
          item => item.meta.ts >= ts_begin && item.meta.ts <= ts_end
        );
      }
      if (this.test_name) {
        items = items.map(item => {
          let perf = 0.0;
          let result = item.results.find(
            result => result[0] === this.test_name
          );
          if (result) {
            result = result[1];
            perf = result[1][1].mean / result[0][1].mean;
          }
          item.perf = perf;
          return item;
        });
      }
      return items;
    },
    chartdata: function() {
      if (this.plot_all) {
        let chartdata = [];
        for (let i = 0, j = this.tests_names.length; i < j; i++) {
          chartdata.push({ name: this.tests_names[i], data: {} });
        }
        for (let i = 0, j = this.tests_names.length; i < j; i++) {
          let test_name = this.tests_names[i];
          this.filtered_items.forEach(function(item) {
            let perf = 0.0;
            let result = item.results.find(result => result[0] === test_name);
            if (result) {
              result = result[1];
              perf = result[1][1].mean / result[0][1].mean;
            }
            chartdata[i].data[new Date(item.meta.ts)] = perf;
          });
        }
        return chartdata;
      } else {
        let chartdata = {};
        this.filtered_items.forEach(function(item) {
          chartdata[new Date(item.meta.ts)] = item.perf;
        });
        return chartdata;
      }
    }
  },
  data() {
    return {
      pagination: { sortBy: "meta.ts", descending: true },
      gitref: null,
      branch: null,
      test_name: null,
      plot_all: false,
      menu_begin: false,
      date_begin: null,
      menu_end: false,
      date_end: null,
      headers: [
        { text: "Date", align: "left", sortable: true, value: "meta.ts" },
        { text: "Author", align: "left", sortable: true, value: "meta.author" },
        {
          text: "Commit",
          align: "left",
          sortable: true,
          value: "commit_id"
        },
        {
          text: "Branch",
          align: "left",
          sortable: true,
          value: "meta.gitref"
        },
        {
          text: "Message",
          align: "left",
          sortable: true,
          value: "meta.message"
        },
        {
          text: "Ratio",
          align: "left",
          sortable: true,
          value: "perf"
        }
      ]
    };
  }
};
</script>