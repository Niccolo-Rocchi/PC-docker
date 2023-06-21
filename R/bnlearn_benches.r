library(bnlearn)
library(microbenchmark)

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
    name <- strsplit(strsplit(df, split = "//")[[1]][2], split=".csv")[[1]][1]
    names <- c(names, name)
    df <- read.csv(df)[-1] |> lapply(factor) |> data.frame()
    mbm <- microbenchmark(pc.stable(df, alpha = 0.05), times = 100L) |> data.frame()
    mbm$expr <- name
    benches <- rbind(benches, mbm)
}

# Save benches results
write.csv(benches, file ="./R/bnlearn_benches.csv")
