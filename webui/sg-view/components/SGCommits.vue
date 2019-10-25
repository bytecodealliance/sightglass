<template>
  <v-container>
    <v-flex>
      <v-layout row wrap>
        <v-autocomplete label="Test:" :items="benchmarks" v-model="benchmark" menu-props="auto" clearable
                        placeholder="Display only results from a specific benchmark test"/>
        <v-autocomplete label="Suite:" :items="suites" v-model="suite" menu-props="auto" clearable
                        placeholder="Display only results from a specific benchmark suite"/>
        <v-autocomplete label="Runtime:" :items="runtimes" v-model="runtime" menu-props="auto" clearable
                        placeholder="Display only results from a specific runtime"/>
        <v-flex sm4>
          <v-switch label="Plot all tests" v-model="plot_all"/>
        </v-flex>
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
    <v-data-table v-bind:pagination.sync="pagination" :headers="headers" :items="tabledata" :loading="loading"
                  class="elevation-1" :rows-per-page-items='[10,25,50,{"text":"All","value":-1}]'>
      <template slot="items" slot-scope="props">
        <td>
          {{ props.item.timestamp | formatDate }}
        </td>
        <td>
          {{ props.item.runtime }}
        </td>
        <td>
          {{ props.item.suite }}
        </td>
        <td>
          <a v-bind:href="props.item.giturl">{{ props.item.git.commit | truncate(8) }}</a>
          {{ props.item.git.message | truncate(30) }} ({{ props.item.git.author }})
        </td>
        <td>
          {{ props.item.slowdown | truncateNumber }}
        </td>
      </template>
    </v-data-table>
  </v-container>
</template>
<script>

  import {calculate_average_slowdown_ratio, extract_target_runtime} from "../js/retrieval";

  export default {
    props: ["loading", "history", "runtimes", "suites", "benchmarks"],
    computed: {
      filtered_items: function () {
        console.debug('Computing filtered items');
        let filtered = Object.values(this.history);

        // filtering by date
        let ts_begin = 0,
          ts_end = Number.MAX_SAFE_INTEGER;
        this.date_begin && (ts_begin = new Date(this.date_begin).getTime());
        this.date_end && (ts_end = new Date(this.date_end).getTime());
        if (ts_begin > 0 && ts_end > 0 && ts_begin < ts_end) {
          filtered = filtered.filter(r => r.meta.timestamp >= ts_begin && r.meta.timestamp <= ts_end);
        }

        // filtering by suite name
        if (this.suite) {
          filtered = filtered.filter(r => r.meta.suite === this.suite);
        }

        // filtering by benchmark name
        if (this.benchmark) {
          filtered = filtered.map(r => {
            return {meta: r.meta, results: {[this.benchmark]: r.results[this.benchmark]}};
          });
        }

        // separate out and filter by runtime
        filtered = filtered.flatMap(r => {
          if (this.runtime) {
            return [extract_target_runtime(this.runtime, r)];
          } else {
            return this.runtimes.map(rt => extract_target_runtime(rt, r))
          }
        });

        // since extract_target_runtime can produce nulls, e.g. when the runtime doesn't exist in the run, we remove
        // them with this filter
        filtered = filtered.filter(r => r);

        return filtered;
      },
      tabledata: function() {
        let tabledata = [];
        for (const run of this.filtered_items) {
          tabledata.push({
            timestamp: run.meta.timestamp,
            runtime: run.runtime,
            suite: run.meta.suite,
            git: run.meta.runtimes[run.runtime],
            giturl: `${run.meta.runtimes[run.runtime].repo}/commits/${run.meta.runtimes[run.runtime].commit}`,
            slowdown: calculate_average_slowdown_ratio(run.runtime, run.meta.reference_runtime, run)
          });
        }
        return tabledata;
      },
      chartdata: function () {
        // for how vue-chartkick expects to receive the data, see examples at https://chartkick.com/vue
        console.debug('Computing chart data');
        if (this.plot_all) {
          // calculate a slowdown line for each found runtime+benchmark combination
          let points = {};
          for (const run of this.filtered_items) {
            for (const benchmark of Object.keys(run.results)){
              for(const runtime of Object.keys(run.results[benchmark])){
                let name = `${benchmark}-${runtime}`;
                points[name] = points[name] || {name: name, data: {}};
                const slowdown = run.results[benchmark][runtime].mean / run.results[benchmark][run.meta.reference_runtime].mean;
                const date = run.meta.timestamp.toISOString();
                points[name].data[date] = slowdown
              }
            }
          }
          return Object.values(points);
        } else {
          // calculate a slowdown line for each found runtime
          let chartdata = [];
          for (const runtime of this.runtimes) {
            let points = {name: runtime, data: {}};
            for (const run of this.filtered_items.filter(r => r.runtime === runtime)) {
              const slowdown = calculate_average_slowdown_ratio(runtime, run.meta.reference_runtime, run);
              const date = run.meta.timestamp.toISOString();
              points.data[date] = slowdown
            }
            chartdata.push(points);
          }
          return chartdata;
        }
      }
    },
    data() {
      return {
        pagination: {sortBy: "meta.ts", descending: true},
        suite: null,
        benchmark: null,
        runtime: null,
        plot_all: false,
        menu_begin: false,
        date_begin: null,
        menu_end: false,
        date_end: null,
        headers: [
          {text: "Date Measured", align: "left", sortable: true, value: "timestamp"},
          {text: "Runtime", align: "left", sortable: true, value: "runtime"},
          {text: "Suite", align: "left", sortable: true, value: "suite"},
          {text: "Commit", align: "left", sortable: true, value: "git"},
          {text: "Slowdown Ratio (lower is better)", align: "left", sortable: true, value: "slowdown"},
        ]
      };
    }
  };
</script>
