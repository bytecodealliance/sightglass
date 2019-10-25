<template>
  <section class="container">
    <div>
      <h1 class="subtitle">
        SightGlass viewer
      </h1>
    </div>
    <SGCommits :loading="loading" :history="history" :benchmarks="benchmarks" :suites="suites" :runtimes="runtimes"/>
  </section>
</template>

<script>
  import SGCommits from "~/components/SGCommits.vue";
  import axios from "axios";
  import {extract_benchmarks, extract_runtimes, extract_suites, fixup_unix_timestamps} from "../js/retrieval";

  let host_history = process.env.HISTORY_URL;
  if (!host_history) {
    if (process.client) {
      host_history = "http://" + window.location.hostname + ":8001/history";
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
          this.history = fixup_unix_timestamps(response.data.history);
          this.benchmarks = extract_benchmarks(this.history);
          this.suites = extract_suites(this.history);
          this.runtimes = extract_runtimes(this.history);
        })
        .catch(e => {
          this.loading = false;
          console.error(e);
          this.errors.push(e);
        });
    },
    data() {
      return {
        loading: true,
        history: {},
        runtimes: [],
        suites: [],
        benchmarks: [],
        errors: [],
      };
    }
  };
</script>
