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
#     $ ./scripts/pca.R ./pca-metrics.csv [native-instruction-budget]
#
# The native-instruction budget defaults to `DEFAULT_NATIVE_INSTRUCTION_BUDGET`;
# see its definition for the derivation.
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
# its member with the lowest combined compilation and execution native
# instruction cost. Sweeping the number of clusters finds the largest
# representative subset whose compilation plus execution native instructions
# fit the supplied budget.
#
# Outputs (written to the current working directory):
#
#   * `scree.svg`: Percentage of variance explained by each principal component.
#   * `cumulative-variance.svg`: running total of variance explained by the
#     first k principal components.
#   * `biplot-1-2.svg`: Biplot of every benchmark on principal components 1 & 2.
#   * `biplot-3-4.svg`: Biplot of every benchmark on principal components 3 & 4.
#   * `biplot-5-6.svg`: Biplot of every benchmark on principal components 5 & 6.
#   * `budget.svg`: native instruction totals for each cluster count, with the
#     largest budget-feasible count marked.
#   * `dendogram.svg`: Dendrogram from the hierarchical clustering of the
#     benchmarks' principal-component scores, with a dotted line at the
#     budget-selected cluster cut.

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

# The metric holding each benchmark's dynamic Wasm instruction count. It is
# used for filtering, not as a PCA feature or native-budget cost.
COST_COLUMN <- "dynamic_total_inst_count"
COMPILATION_NATIVE_INSTRUCTIONS_COLUMN <- "compilation_native_instructions"
EXECUTION_NATIVE_INSTRUCTIONS_COLUMN <- "execution_native_instructions"
NATIVE_INSTRUCTION_COLUMNS <- c(
    COMPILATION_NATIVE_INSTRUCTIONS_COLUMN,
    EXECUTION_NATIVE_INSTRUCTIONS_COLUMN
)
DEFAULT_NATIVE_INSTRUCTION_BUDGET <- (
    5      # est. insts per ns
    * 1000 # est. insts per us
    * 1000 # est. insts per ms
    * 1000 # est. insts per second
    * 60   # est. insts per min
    * 10   # est. insts per 10-minute, 30-iteration run
    / 30   # est. insts per iteration
)

# This target sets the desired workload size and the filtering floor below.
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
    required <- c(COST_COLUMN, NATIVE_INSTRUCTION_COLUMNS)
    missing <- setdiff(required, names(df))
    if (length(missing) > 0) {
        stop(sprintf(
            "expected required metric columns: %s",
            paste(sprintf("'%s'", missing), collapse = ", ")
        ))
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
# This is every numeric column except the filtering/representative-size metric,
# raw native instruction count columns, and any constant column (a constant
# metric carries no information for PCA and would make per-variable scaling
# divide by zero).
pca_features <- function(data) {
    numeric_cols <- names(data)[vapply(data, is.numeric, logical(1))]
    excluded <- c(COST_COLUMN, NATIVE_INSTRUCTION_COLUMNS)
    features <- data[, setdiff(numeric_cols, excluded), drop = FALSE]
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

# Index, within a cluster's native instruction counts, of the representative:
# the member with the minimum combined compilation and execution cost.
representative_index <- function(compilation, execution) {
    which.min(compilation + execution)
}

# Return the original row indices of a cluster assignment's representatives.
representative_indices <- function(compilation, execution, assignment) {
    clusters <- split(seq_along(assignment), assignment)
    vapply(clusters, function(idx) {
        idx[representative_index(compilation[idx], execution[idx])]
    }, integer(1))
}

# Sum the compilation, execution, and combined native costs of representatives.
representative_native_costs <- function(compilation, execution, assignment) {
    indices <- representative_indices(compilation, execution, assignment)
    compilation_total <- sum(compilation[indices])
    execution_total <- sum(execution[indices])
    list(
        indices = indices,
        compilation = compilation_total,
        execution = execution_total,
        combined = compilation_total + execution_total
    )
}

# Group benchmarks by cluster for the suggested-subset report.
#
# Each cluster's complete member table is sorted by combined native instructions
# so the selected minimum-cost representative appears first.
cluster_members <- function(assignment, cost, compilation, execution, paths) {
    lapply(split(seq_along(assignment), assignment), function(idx) {
        members <- data.frame(
            benchmark = paths[idx],
            dynamic_insts = cost[idx],
            compilation_native_insts = compilation[idx],
            execution_native_insts = execution[idx],
            stringsAsFactors = FALSE
        )
        members$combined_native_insts <- (
            members$compilation_native_insts + members$execution_native_insts
        )
        members[order(members$combined_native_insts, members$dynamic_insts), , drop = FALSE]
    })
}

# Select the greatest cluster count whose representative total fits the budget.
select_budget_cluster_count <- function(clusters, combined_cost, budget) {
    one_cluster_cost <- combined_cost[clusters == 1]
    if (length(one_cluster_cost) != 1 || one_cluster_cost > budget) {
        stop(sprintf(
            "native instruction budget (%s) is too small: no one-cluster subset fits",
            format(budget, big.mark = ",", scientific = FALSE)
        ))
    }
    feasible <- combined_cost <= budget
    max(clusters[feasible])
}

# Sweep every possible cluster count and retain the largest budget-feasible
# representative subset. SSE is retained only as a clustering diagnostic.
budget_analysis <- function(scores, clustering, compilation, execution, budget) {
    ks <- seq_len(nrow(scores))
    assignments <- lapply(ks, function(k) cutree(clustering, k = k))
    sse <- vapply(assignments, function(a) within_cluster_sse(scores, a), numeric(1))
    representative_costs <- lapply(
        assignments,
        function(a) representative_native_costs(compilation, execution, a)
    )
    compilation_totals <- vapply(representative_costs, `[[`, numeric(1), "compilation")
    execution_totals <- vapply(representative_costs, `[[`, numeric(1), "execution")
    combined_totals <- vapply(representative_costs, `[[`, numeric(1), "combined")
    best_k <- select_budget_cluster_count(ks, combined_totals, budget)
    list(
        clusters = ks,
        sse = sse,
        compilation = compilation_totals,
        execution = execution_totals,
        combined = combined_totals,
        representative_costs = representative_costs,
        best_k = best_k,
        budget = budget
    )
}

# Write native instruction totals at each cluster count.
write_budget_plot <- function(analysis) {
    best <- analysis$best_k
    data <- data.frame(
        clusters = analysis$clusters,
        compilation = analysis$compilation,
        execution = analysis$execution,
        combined = analysis$combined
    )
    selected <- data[data$clusters == best, ]
    cat(sprintf(
        "Budget-selected cluster size: %d clusters within %s native instructions.\n",
        best, format(analysis$budget, big.mark = ",", scientific = FALSE)
    ))

    plot <- ggplot(data, aes(x = clusters, y = combined / 1e9)) +
        geom_line(color = "steelblue") +
        geom_point(color = "steelblue", size = 0.9) +
        geom_hline(yintercept = analysis$budget / 1e9, linetype = "dashed", color = "gray40") +
        geom_point(data = selected, color = "red", size = 2.5) +
        annotate(
            "text", x = selected$clusters, y = selected$combined / 1e9,
            label = sprintf("  Selected: %d clusters", best),
            hjust = 0, color = "red"
        ) +
        labs(
            title = "Native Instruction Budget by Cluster Count",
            x = "Cluster count",
            y = "Representative compilation + execution native instructions (billions)"
        ) +
        theme_minimal()
    ggsave("budget.svg", plot = plot, width = 12, height = 6)
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
# the budget-selected cluster cut.
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

parse_args <- function(args) {
    usage <- paste0(
        "usage: ./scripts/pca.R <metrics.csv> [native-instruction-budget]\n",
        "native-instruction-budget must be a single finite, positive whole number"
    )
    if (length(args) < 1 || length(args) > 2) {
        stop(usage)
    }
    budget <- DEFAULT_NATIVE_INSTRUCTION_BUDGET
    if (length(args) == 2) {
        value <- args[[2]]
        if (!grepl("^[0-9]+$", value)) {
            stop(usage)
        }
        budget <- suppressWarnings(as.numeric(value))
        if (!is.finite(budget) || budget <= 0 || budget != floor(budget)) {
            stop(usage)
        }
    }
    list(metrics_path = args[[1]], budget = budget)
}

main <- function() {
    args <- parse_args(commandArgs(trailingOnly = TRUE))
    data <- read_data(args$metrics_path)
    # The representative-size metric, native instruction counts, and full
    # benchmark paths, aligned with the data's row order.
    cost <- data[[COST_COLUMN]]
    compilation <- data[[COMPILATION_NATIVE_INSTRUCTIONS_COLUMN]]
    execution <- data[[EXECUTION_NATIVE_INSTRUCTIONS_COLUMN]]
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

    # Find the largest subset that fits the budget; the dendrogram marks the
    # same cut.
    analysis <- budget_analysis(
        scores, clustering, compilation, execution, args$budget
    )

    write_scree_plot(pca)
    write_cumulative_variance_plot(propve)
    write_biplot(pca, c(1, 2), "biplot-1-2.svg")
    write_biplot(pca, c(3, 4), "biplot-3-4.svg")
    write_biplot(pca, c(5, 6), "biplot-5-6.svg")
    write_budget_plot(analysis)
    write_dendrogram(clustering, analysis$best_k)

    assignment <- cutree(clustering, k = analysis$best_k)
    selected <- representative_native_costs(compilation, execution, assignment)
    clusters <- cluster_members(assignment, cost, compilation, execution, paths)
    remaining <- args$budget - selected$combined
    cat(sprintf(
        paste0("\nSuggested subset: the benchmark with the minimum combined ",
               "native instruction cost in each of the %d clusters:\n\n"),
        analysis$best_k
    ))
    cat(sprintf(
        paste0("Native instruction budget: %s\n",
               "Selected compilation native instructions: %s\n",
               "Selected execution native instructions: %s\n",
               "Selected combined native instructions: %s\n",
               "Remaining native instruction budget: %s\n\n"),
        format(args$budget, big.mark = ",", scientific = FALSE),
        format(selected$compilation, big.mark = ",", scientific = FALSE),
        format(selected$execution, big.mark = ",", scientific = FALSE),
        format(selected$combined, big.mark = ",", scientific = FALSE),
        format(remaining, big.mark = ",", scientific = FALSE)
    ))
    cat("```\n")
    for (n in seq_along(selected$indices)) {
        i <- selected$indices[[n]]
        members <- clusters[[n]]
        if (n > 1) {
            cat("\n")
        }
        cat(sprintf("# Cluster %d\n", n - 1L))
        cat("#\n")
        cat(paste0(
            "#",
            sprintf("%16s", "Dynamic Wasm"),
            sprintf("%22s", "Compilation Native"),
            sprintf("%20s", "Execution Native"),
            sprintf("%19s", "Combined Native"),
            "    Benchmark\n"
        ))
        cat(paste0("# ", strrep("-", 118), "\n"))
        for (j in seq_len(nrow(members))) {
            cat(paste0(
                "#",
                sprintf("%16s", format(members$dynamic_insts[j],
                                       big.mark = ",", scientific = FALSE)),
                sprintf("%22s", format(members$compilation_native_insts[j],
                                       big.mark = ",", scientific = FALSE)),
                sprintf("%20s", format(members$execution_native_insts[j],
                                       big.mark = ",", scientific = FALSE)),
                sprintf("%19s", format(members$combined_native_insts[j],
                                       big.mark = ",", scientific = FALSE)),
                "    ", members$benchmark[j], "\n"
            ))
        }
        cat(paths[i], "\n", sep = "")
    }
    cat("```\n")
}

if (sys.nframe() == 0) {
    main()
}
