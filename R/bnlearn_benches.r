library(bnlearn)
library(microbenchmark)
library(parallel)

# Read file names
csv <- list.files(
  path = "./tests/assets/PC-Stable/",
  pattern = "\\.csv$",
  full.names = TRUE
)

# Benches
names <- c()
benches <- data.frame()
for (df in csv) {
    # Get dataframe
    name <- strsplit(strsplit(df, split = "//")[[1]][2], split=".csv")[[1]][1]
    names <- c(names, name)
    df <- read.csv(df)[-1] |> lapply(factor) |> data.frame()
    # Single core benches
    mbm <- microbenchmark(pc.stable(x = df, alpha = 0.05), times = 100L) |> data.frame()
    mbm$expr <- name
    benches <- rbind(benches, mbm)
    # Multi core benches
    fork_cluster <- makeForkCluster(nnodes = detectCores())
    par_mbm <- microbenchmark(pc.stable(x= df, cluster = fork_cluster, alpha = 0.05), times = 100L) |> data.frame()
    par_mbm$expr <- paste0("par_", name)
    benches <- rbind(benches, par_mbm)
}

# Save benches results
write.csv(benches, file ="./R/bnlearn_benches.csv")
