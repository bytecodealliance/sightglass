#!/usr/bin/Rscript --vanilla

# Principal component analysis (PCA) and agglomerative hierarchical clustering
# of Sightglass benchmark metrics.
#
# Before running the first time:
#
#     $ R
#     > install.packages(c("FactoMineR", "factoextra", "svglite", "ggplot2"))
#
# Usage:
#
#     $ cargo run -- pca-metrics -o ./pca-metrics.csv -- benchmarks...
#     $ ./scripts/pca.R ./pca-metrics.csv
#
# The methodology is based on "A Workload Characterization of the SPEC CPU2017
# Benchmark Suite" by Limaye and Adegbija:
#
#     https://tosiron.com/papers/2018/SPEC2017_ISPASS18.pdf
#
# Each metric is standardized (centered to mean 0, scaled to unit variance) and
# PCA is run on the resulting correlation matrix so that metrics measured on
# different scales contribute comparably. Benchmarks are then clustered by the
# Euclidean distance between their principal-component scores, as in the paper.
#
# Finally, we recommend a subset of the suite. Each cluster is represented by
# the member whose dynamic wasm instruction count is closest to
# `TARGET_INST_COUNT`, so the subset runs benchmarks of a representative,
# substantial size rather than each cluster's briefest (and noisiest) member.
# Sweeping the number of clusters traces a Pareto trade-off between clustering
# error (SSE) and the cost of running the subset (its total dynamic
# instructions); the knee of that curve is the Pareto-optimal cluster size.
#
# Outputs (written to the current working directory):
#
#   * `scree.svg`: Percentage of variance explained by each principal component.
#   * `cumulative-variance.svg`: running total of variance explained by the
#     first k principal components.
#   * `biplot-1-2.svg`: Biplot of every benchmark on principal components 1 & 2.
#   * `biplot-3-4.svg`: Biplot of every benchmark on principal components 3 & 4.
#   * `biplot-5-6.svg`: Biplot of every benchmark on principal components 5 & 6.
#   * `pareto.svg`: SSE vs subset execution cost as the cluster count varies,
#     with the Pareto-optimal cluster size marked.
#   * `dendogram.svg`: Dendrogram from the hierarchical clustering of the
#     benchmarks' principal-component scores, with a dotted line at the
#     Pareto-optimal cluster cut.
#   * `clusters.csv`: one `wasm,cluster` row per (post-filtering) benchmark,
#     giving the cluster it lands in at the Pareto-optimal cut.

library("FactoMineR")
library("factoextra")
library("svglite")
library("ggplot2")

# Draw every individual's label, even where benchmarks land on top of each
# other (e.g. near-identical libsodium variants). Without this, ggrepel silently
# drops labels once too many overlap.
options(ggrepel.max.overlaps = Inf)

# A biplot overlays the individuals (benchmarks) with the variables (metrics) as
# loading arrows. With ~50 metrics, drawing every arrow buries the individuals,
# so each biplot only draws arrows for the variables that contribute most to the
# pair of axes it shows.
N_BIPLOT_VARS <- 12

# The principal components retained for clustering must capture at least this
# fraction of the total variance. The components with lower variance carry
# little information and can be dropped without significant loss.
CLUSTER_VAR_THRESHOLD <- 0.9

# The metric holding each benchmark's dynamic instruction count. This is the
# execution-cost axis of the Pareto subsetting analysis, not a workload
# characteristic, so it is excluded from the PCA itself.
COST_COLUMN <- "dynamic_total_inst_count"

# Each cluster is represented by the benchmark whose dynamic instruction count
# is closest to this target, rather than by its cheapest member. Choosing a
# representative near this size keeps the subset's benchmarks long enough to be
# meaningful while steering away from each cluster's most expensive members.
TARGET_INST_COUNT <- 100000000

# Benchmarks executing fewer than this many dynamic instructions run too briefly
# to characterize reliably so they are filtered out before the analysis. The
# floor is half the representative target: a benchmark smaller than that is too
# far below the target size to stand in for its cluster anyway.
MIN_DYNAMIC_INST_COUNT <- TARGET_INST_COUNT / 2

# Turn a benchmark path into a short, unique label.
#
# Strip the shared "benchmarks/" prefix and the ".wasm" suffix and keep the
# file's stem, e.g.  "benchmarks/spidermonkey/spidermonkey-json.wasm" ->
# "spidermonkey-json". Files named the generic "benchmark.wasm" have no
# information in their stem, so fall back to the parent directory:
# "benchmarks/richards/benchmark.wasm" -> "richards".
benchmark_labels <- function(names) {
    stripped <- sub("^benchmarks/", "", names)
    vapply(strsplit(stripped, "/", fixed = TRUE), function(parts) {
        stem <- sub("[.]wasm$", "", parts[length(parts)])
        if (stem == "benchmark" && length(parts) >= 2) {
            parts[length(parts) - 1L]
        } else {
            stem
        }
    }, character(1))
}

# Read the metrics CSV into a data frame whose rows are individual benchmarks,
# named by their short label.
#
# The full-path `benchmark` column and all numeric metrics (including the
# `COST_COLUMN`) are kept. Benchmarks executing fewer than
# `MIN_DYNAMIC_INST_COUNT` dynamic instructions are dropped (with a warning)
# because executing too few instructions makes them noisy.
read_data <- function(file_path) {
    df <- read.csv(file_path)
    if (!(COST_COLUMN %in% names(df))) {
        stop(sprintf("expected a '%s' column for the SSE/cost trade-off", COST_COLUMN))
    }

    # Label each row (individual) by a short version of its benchmark name; the
    # original full path stays in the `benchmark` column for later reporting.
    rownames(df) <- benchmark_labels(df$benchmark)

    cat("\n")
    below <- df[[COST_COLUMN]] < MIN_DYNAMIC_INST_COUNT
    for (i in which(below)) {
        warning(sprintf(
            "filtering out %s: only %s dynamic instructions (< %s)",
            df$benchmark[i],
            format(df[[COST_COLUMN]][i], big.mark = ",", scientific = FALSE),
            format(MIN_DYNAMIC_INST_COUNT, big.mark = ",", scientific = FALSE)
        ), call. = FALSE, immediate. = TRUE)
    }
    df[!below, , drop = FALSE]
}

# Get the characterization metrics fed to PCA.
#
# This is every numeric column except the execution-cost column and any constant
# column (a constant metric carries no information for PCA and would make
# per-variable scaling divide by zero).
pca_features <- function(data) {
    numeric_cols <- names(data)[vapply(data, is.numeric, logical(1))]
    features <- data[, setdiff(numeric_cols, COST_COLUMN), drop = FALSE]
    informative <- vapply(features, function(col) {
        v <- var(col)
        !is.na(v) && v > 0
    }, logical(1))
    features[, informative, drop = FALSE]
}

# Write the scree plot.
#
# The x axis is each principal component, y axis is the percentage of the total
# variance that component explains.
write_scree_plot <- function(pca) {
    n <- length(pca$sdev)
    plot <- fviz_eig(
        pca,
        choice = "variance",
        # Show every component, not just the default leading ten, so the full
        # decay of explained variance is visible.
        ncp = n,
        addlabels = TRUE,
        geom = c("bar", "line"),
        barfill = "steelblue",
        barcolor = "steelblue",
        main = "Scree Plot",
        xlab = "Principal Component",
        ylab = "Percentage of Variance Explained"
    )
    ggsave("scree.svg", plot = plot, width = 12, height = 6)
}

# Write the cumulative variance plot
#
# This is the sibling of the scree plot, but the y axis is the running total of
# variance explained by the first k principal components. The dashed line marks
# `CLUSTER_VAR_THRESHOLD`, the cutoff used to decide how many PCs feed the
# hierarchical clustering.
write_cumulative_variance_plot <- function(propve) {
    components <- seq_along(propve)
    data <- data.frame(
        component = components,
        cumulative = 100 * cumsum(propve)
    )

    # Mirror fviz_eig's scree styling (steelblue bars topped by a line + points)
    # so the two figures read as a pair.
    plot <- ggplot(data, aes(x = component, y = cumulative)) +
        geom_col(fill = "steelblue", color = "steelblue") +
        geom_line(color = "black") +
        geom_point(color = "black", size = 1) +
        geom_hline(
            yintercept = 100 * CLUSTER_VAR_THRESHOLD,
            linetype = "dashed", color = "gray40"
        ) +
        scale_x_continuous(breaks = components) +
        scale_y_continuous(limits = c(0, 100)) +
        labs(
            title = "Cumulative Variance Plot",
            x = "Principal Component",
            y = "Cumulative Percentage of Variance Explained"
        ) +
        theme_minimal()
    ggsave("cumulative-variance.svg", plot = plot, width = 12, height = 6)
}

# Write a biplot of the two requested principal components.
#
# Every benchmark is plotted as a labeled point, plus loading arrows for the
# metrics that most shape these axes.
write_biplot <- function(pca, axes, file_path) {
    plot <- fviz_pca_biplot(
        pca,
        axes = axes,
        # Plot and label every individual benchmark.
        geom.ind = c("point", "text"),
        col.ind = "gray40",
        pointsize = 1,
        labelsize = 2,
        # Only the strongest contributors to this pair of axes get an arrow.
        select.var = list(contrib = N_BIPLOT_VARS),
        col.var = "contrib",
        gradient.cols = c("#00AFBB", "#E7B800", "#FC4E07"),
        repel = TRUE,
        title = sprintf("Biplot: PC%d vs PC%d", axes[1], axes[2])
    )
    ggsave(file_path, plot = plot, width = 14, height = 14)
}

# Get the total within-cluster sum of squared errors.
#
# This is, for each cluster, the sum of squared Euclidean distances from its
# members to the cluster centroid (in PC-score space).
#
# SSE falls as the number of clusters grows (because there are fewer individuals
# per cluster; in the limit, one individual per cluster means there will be no
# error as the cluster is centered exactly on its individual).
within_cluster_sse <- function(scores, assignment) {
    clusters <- split(seq_along(assignment), assignment)
    sum(vapply(clusters, function(idx) {
        members <- scores[idx, , drop = FALSE]
        centroid <- colMeans(members)
        sum(rowSums(sweep(members, 2, centroid)^2))
    }, numeric(1)))
}

# Index, within a vector of dynamic instruction counts, of a cluster's
# representative: the member whose count is closest to `TARGET_INST_COUNT`.
representative_index <- function(cost) {
    which.min(abs(cost - TARGET_INST_COUNT))
}

# Cost of representing every cluster by its representative member.
#
# Each cluster's representative -- the benchmark whose dynamic instruction count
# is closest to `TARGET_INST_COUNT` -- stands in for the whole cluster, so this
# total is the sum of the representatives' instruction counts.
subset_cost <- function(cost, assignment) {
    clusters <- split(seq_along(assignment), assignment)
    sum(vapply(clusters, function(idx) {
        cluster_cost <- cost[idx]
        cluster_cost[representative_index(cluster_cost)]
    }, numeric(1)))
}

# Group benchmarks by cluster at size k.
#
# Returns a list with one data frame per cluster: member full paths and dynamic
# instruction counts, sorted ascending by count for a readable cost breakdown.
cluster_members <- function(clustering, cost, paths, k) {
    assignment <- cutree(clustering, k = k)
    lapply(split(seq_along(assignment), assignment), function(idx) {
        members <- data.frame(
            benchmark = paths[idx],
            dynamic_insts = cost[idx],
            stringsAsFactors = FALSE
        )
        members[order(members$dynamic_insts), , drop = FALSE]
    })
}

# Get the knee of a two-objective trade-off curve.
#
# This is the point whose perpendicular distance from the chord joining the
# curve's endpoints is greatest.
#
# Both axes are normalized to [0, 1] first so the distance does not depend on
# their units.
knee_index <- function(x, y) {
    nx <- (x - min(x)) / (max(x) - min(x))
    ny <- (y - min(y)) / (max(y) - min(y))
    last <- length(nx)
    dx <- nx[last] - nx[1]
    dy <- ny[last] - ny[1]
    distance <- abs(dy * nx - dx * ny + nx[last] * ny[1] - ny[last] * nx[1]) /
        sqrt(dx^2 + dy^2)
    which.max(distance)
}

# Sweep every possible cluster count, recording the clustering error (SSE) and
# the subset's execution cost at each, and pick the Pareto-optimal count (the
# knee of SSE vs cost).
pareto_analysis <- function(scores, clustering, cost) {
    ks <- seq_len(nrow(scores))
    assignments <- lapply(ks, function(k) cutree(clustering, k = k))
    sse <- vapply(assignments, function(a) within_cluster_sse(scores, a), numeric(1))
    subset_insts <- vapply(assignments, function(a) subset_cost(cost, a), numeric(1))
    list(
        clusters = ks,
        sse = sse,
        cost = subset_insts,
        best_k = ks[knee_index(subset_insts, sse)]
    )
}

# Write a plot of the Pareto curve.
#
# This plots clustering error (SSE) against the subset's execution cost, one
# point per cluster count, with the Pareto-optimal count highlighted.
write_pareto_plot <- function(analysis) {
    best <- analysis$best_k
    data <- data.frame(
        clusters = analysis$clusters,
        sse = analysis$sse,
        cost = analysis$cost
    )
    optimal <- data[data$clusters == best, ]
    cat(sprintf(
        paste0("Pareto-optimal cluster size: %d clusters; the subset runs ",
               "%.1f%% of the suite's dynamic instructions.\n"),
        best, 100 * optimal$cost / max(data$cost)
    ))

    plot <- ggplot(data, aes(x = cost / 1e9, y = sse)) +
        geom_line(color = "steelblue") +
        geom_point(color = "steelblue", size = 0.9) +
        geom_point(data = optimal, color = "red", size = 2.5) +
        annotate(
            "text", x = optimal$cost / 1e9, y = optimal$sse,
            label = sprintf("  Pareto-optimal: %d clusters", best),
            hjust = 0, color = "red"
        ) +
        labs(
            title = "Pareto-Optimal Cluster Size (clustering error vs execution cost)",
            x = "Total dynamic wasm instructions executed by the subset (billions)",
            y = "Within-cluster sum of squared errors (SSE)"
        ) +
        theme_minimal()
    ggsave("pareto.svg", plot = plot, width = 12, height = 6)
}

# The dendrogram height at which cutting the tree yields exactly `k` clusters:
# midway between the (n-k)th and (n-k+1)th merge heights.
cut_height_for_k <- function(clustering, k) {
    heights <- sort(clustering$height)
    i <- length(heights) + 1L - k
    below <- if (i >= 1L && i <= length(heights)) heights[i] else 0
    above <- if (i + 1L <= length(heights)) heights[i + 1L] else max(heights)
    mean(c(below, above))
}

# Write a dendrogram of the hierarchical clustering, with a dotted line marking
# the Pareto-optimal cluster cut.
write_dendrogram <- function(clustering, best_k) {
    cut_height <- cut_height_for_k(clustering, best_k)

    plot <- fviz_dend(
        clustering,
        horiz = TRUE,
        cex = 0.5,
        # Thinner branches than the default 0.7 so the tree doesn't look heavy.
        lwd = 0.4,
        main = sprintf(
            "Hierarchical Clustering of Benchmarks (dotted line = %d-cluster cut)",
            best_k
        )
    ) +
        # fviz_dend maps branch width through a continuous linewidth scale, which
        # inflates `lwd` into a fat default range (and leaks a stray legend).
        # Render the width as-is so the thin `lwd` above actually takes effect.
        scale_linewidth_identity() +
        # fviz_dend draws horizontal trees by flipping the axes, so a horizontal
        # line on the height axis renders as the vertical cut line we want.
        geom_hline(yintercept = cut_height, linetype = "dotted", color = "red") +
        theme(legend.position = "none")

    # 127 leaves need a tall canvas to keep the labels legible; disable the
    # default dimension sanity check that would otherwise reject it.
    ggsave("dendogram.svg", plot = plot, width = 12, height = 24, limitsize = FALSE)
}

# Write `clusters.csv`: one row per (post-filtering) benchmark giving its full
# wasm path and the cluster it falls in at the Pareto-optimal cut.
write_clusters_csv <- function(clustering, paths, best_k) {
    assignment <- cutree(clustering, k = best_k)
    out <- data.frame(
        wasm = paths,
        cluster = as.integer(assignment) - 1L,
        stringsAsFactors = FALSE
    )
    # Group the rows by cluster so the file reads nicely.
    out <- out[order(out$cluster, out$wasm), , drop = FALSE]
    write.csv(out, "clusters.csv", row.names = FALSE, quote = FALSE)
}

main <- function() {
    args <- commandArgs(trailingOnly = TRUE)
    if (length(args) < 1) {
        stop("usage: ./scripts/pca.R <metrics.csv>")
    }

    data <- read_data(args[1])
    # The execution-cost vector and full benchmark paths, aligned with the data's
    # row order.
    cost <- data[[COST_COLUMN]]
    paths <- data$benchmark

    # Standardize each metric and run PCA on the correlation matrix. `retx`
    # keeps the rotated data (the per-benchmark PC scores) in `pca$x`.
    pca <- prcomp(pca_features(data), center = TRUE, scale. = TRUE, retx = TRUE)

    # Proportion of variance explained by each component.
    variances <- pca$sdev^2
    propve <- variances / sum(variances)

    # Retain the leading PCs that together explain `CLUSTER_VAR_THRESHOLD` of
    # the variance, then cluster on those scores (Euclidean distance, Ward's
    # linkage, which favors compact, well-separated clusters). Clustering on the
    # PCs rather than the raw metrics drops the low-variance, noisy directions.
    cumulative <- cumsum(propve)
    n_keep <- which(cumulative >= CLUSTER_VAR_THRESHOLD)[1]
    cat(sprintf(
        "\nClustering on the first %d principal components (%.1f%% of variance).\n",
        n_keep, 100 * cumulative[n_keep]
    ))
    scores <- pca$x[, seq_len(n_keep), drop = FALSE]
    clustering <- hclust(dist(scores, method = "euclidean"), method = "ward.D2")

    # Find the Pareto-optimal cluster size; the dendrogram marks the same cut.
    analysis <- pareto_analysis(scores, clustering, cost)

    write_scree_plot(pca)
    write_cumulative_variance_plot(propve)
    write_biplot(pca, c(1, 2), "biplot-1-2.svg")
    write_biplot(pca, c(3, 4), "biplot-3-4.svg")
    write_biplot(pca, c(5, 6), "biplot-5-6.svg")
    write_pareto_plot(analysis)
    write_dendrogram(clustering, analysis$best_k)
    write_clusters_csv(clustering, paths, analysis$best_k)

    # With the graphs written, report the suggested subset per cluster.
    clusters <- cluster_members(clustering, cost, paths, analysis$best_k)
    cat(sprintf(
        paste0("\nSuggested subset: the benchmark closest to %s dynamic ",
               "instructions in each of the %d clusters:\n\n"),
        format(TARGET_INST_COUNT, big.mark = ",", scientific = FALSE),
        analysis$best_k
    ))
    cat("```\n")
    for (n in seq_along(clusters)) {
        members <- clusters[[n]]
        if (n > 1) {
            cat("\n")
        }
        cat(sprintf("# Cluster %d\n", n - 1L))
        cat("#\n")
        cat(paste0("#", sprintf("%16s", "Instructions"), "    Benchmark\n"))
        cat(paste0("# ", strrep("-", 78), "\n"))
        for (j in seq_len(nrow(members))) {
            cat(paste0(
                "#",
                sprintf("%16s", format(members$dynamic_insts[j],
                                       big.mark = ",", scientific = FALSE)),
                "    ", members$benchmark[j], "\n"
            ))
        }
        # The representative: the member whose instruction count is closest to
        # `TARGET_INST_COUNT`.
        rep_idx <- representative_index(members$dynamic_insts)
        cat(members$benchmark[rep_idx], "\n", sep = "")
    }
    cat("```\n")
}

main()
