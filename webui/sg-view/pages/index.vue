<template>
  <section class="container">
    <div>
      <h1 class="subtitle">
        SightGlass viewer
      </h1>
    </div>
    <SGCommits :loading="loading" :items="items" :tests_names="tests_names" :gitrefs="gitrefs" />
  </section>
</template>

<script>
import SGCommits from "~/components/SGCommits.vue";
import axios from "axios";
import dayjs from "dayjs";

export default {
  components: {
    SGCommits
  },
  created() {
    axios
      .get("http://localhost:8001/history")
      .then(response => {
        this.loading = false;
        let items_obj = response.data.history;
        let items = [];
        let tests_names = [];
        let gitrefs = [];
        for (var commit_id in items_obj) {
          let commit = items_obj[commit_id];
          let avg_cretonne = 0;
          let avg_ref = 0;
          let avg_count = 0;
          let results = commit.results;
          for (var i = 0, j = results.length; i < j; i++) {
            let test_name = results[i][0];
            let xperf_cretonne = results[i][1][1][1];
            let xperf_ref = results[i][1][0][1];
            tests_names.includes(test_name) || tests_names.push(test_name);
            avg_cretonne += xperf_cretonne.mean;
            avg_ref += xperf_ref.mean;
            avg_count++;
          }
          commit.meta.ts = commit.meta.ts.secs_since_epoch * 1000;
          avg_cretonne = Math.round(avg_cretonne / avg_count);
          avg_ref = Math.round(avg_ref / avg_count);
          commit.meta.gitref = commit.meta.gitref.replace(/^refs\/heads\//, "");
          gitrefs.includes(commit.meta.gitref) ||
            gitrefs.push(commit.meta.gitref);
          items.push({
            commit_id: commit_id,
            meta: commit.meta,
            results: commit.results,
            perf: avg_cretonne / avg_ref
          });
        }
        this.tests_names = tests_names;
        this.gitrefs = gitrefs;
        this.items = items;
      })
      .catch(e => {
        this.loading = false;
        this.errors.push(e);
        console.error(e);
      });
  },
  data() {
    return {
      loading: true,
      items: [],
      chartdata: {},
      tests_names: [],
      test_name: null,
      gitrefs: [],
      gitref: null,
      errors: []
    };
  }
};
</script>
