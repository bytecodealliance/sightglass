import Vue from "vue";
import numeral from "numeral";

Vue.filter("truncateNumber", value => numeral(value).format("0.000"));
