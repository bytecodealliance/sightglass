<template>
  <section class="container">
    <div>
      <h1 class="subtitle">
        SightGlass viewer
      </h1>
    </div>
    <SGCommits :loading="loading" :items="items" :tests_names="tests_names" :gitrefs="gitrefs"/>
  </section>
</template>

<script>
  import SGCommits from "~/components/SGCommits.vue";
  import axios from "axios";
  import {
      calculate_average_slowdown_ratio,
      convert_meta,
      extract_git_refs,
      extract_test_names,
      get_index_by_name
  } from "../js/retrieval";

  let host_history = process.env.HISTORY_URL;
  if (!host_history) {
    if (process.client) {
      host_history = "http://"+window.location.hostname+":8001/history";
    } else {
      host_history = "http://localhost:8001/history";
    }
  }

  export default {
    components: {
      SGCommits
    },
    created() {
      axios
        .get(host_history)
        .then(response => {
          this.loading = false;
          let items_obj = response.data.history;

          let items = [];
          for (let [commit, result] of Object.entries(items_obj)) {
            const lucet_index = get_index_by_name("lucet", result.results);
            const reference_index = get_index_by_name("Reference", result.results);
            items.push({
              commit_id: commit,
              meta: convert_meta(result.meta),
              results: result.results,
              perf: calculate_average_slowdown_ratio(lucet_index, reference_index, result.results)
            });
          }

          this.tests_names = extract_test_names(items_obj);
          this.gitrefs = extract_git_refs(items_obj);
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
