// Copied and adapted from ./benchmarks/benchmark.cc of the intgemm.
#include "sightglass.h"
#include "./intgemm/aligned.h"
#include "intgemm/intgemm_config.h"
#include "./intgemm/sse2_gemm.h"
#include "./intgemm/ssse3_gemm.h"
#include "./intgemm/intgemm.h"
#include "./intgemm/stats.h"
#include "./intgemm/callbacks.h"

#include <algorithm>
#include <cassert>
#include <chrono>
#include <cmath>
#include <cstdint>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <iomanip>
#include <iostream>
#include <random>

// This benchmark is built with Emscripten, whose libc filesystem does not reach
// WASI preopened directories. So, like the splay and blake3-simd benchmarks, we
// read the workload size from disk by calling the WASI `path_open`/`fd_read`
// syscalls directly. This lets the workload be resized without recompiling.
#define WASI_IMPORT(name) \
    __attribute__((import_module("wasi_snapshot_preview1"), import_name(name)))

typedef struct {
    const void *buf;
    size_t len;
} wasi_iovec_t;

WASI_IMPORT("path_open")
int wasi_path_open(int fd, int dirflags, const char *path, size_t path_len,
                   int oflags, uint64_t rights_base, uint64_t rights_inheriting,
                   int fdflags, int *opened_fd);

WASI_IMPORT("fd_read")
int wasi_fd_read(int fd, const wasi_iovec_t *iovs, size_t iovs_len, size_t *nread);

static int read_int_from_file()
{
    const char *path = "default.input"; // preopen fd 3 is the benchmark dir
    int fd = -1;
    if (wasi_path_open(3, 0, path, strlen(path), 0, (1ULL << 1) | (1ULL << 2),
                       (1ULL << 1) | (1ULL << 2), 0, &fd) != 0 || fd < 0) {
        std::cerr << "failed to open default.input" << std::endl;
        abort();
    }
    char buf[64] = {0};
    size_t total = 0;
    for (;;) {
        wasi_iovec_t iov = {buf + total, sizeof(buf) - 1 - total};
        size_t nread = 0;
        if (wasi_fd_read(fd, &iov, 1, &nread) != 0) { abort(); }
        if (nread == 0 || total >= sizeof(buf) - 1) break;
        total += nread;
    }
    buf[total] = '\0';
    return atoi(buf);
}

namespace intgemm {
namespace {

struct RandomMatrices {
  RandomMatrices(Index A_rows_in, Index width_in, Index B_cols_in) :
    A_rows(A_rows_in), width(width_in), B_cols(B_cols_in),
    A(A_rows * width), B(width * B_cols) {
    std::mt19937 gen;
    std::uniform_real_distribution<float> dist(-1.f, 1.f);
    gen.seed(45678);

    for (auto& it : A) {
      it = dist(gen);
    }
    for (auto& it : B) {
      it = dist(gen);
    }
  }

  const Index A_rows, width, B_cols;
  AlignedVector<float> A, B;
};

template <class Backend> double Run(const RandomMatrices &m) {
  using Integer = typename Backend::Integer;
  float quant_mult = 127.0f / 2.0f;
  float unquant_mult = 1.0f / (quant_mult * quant_mult);
  AlignedVector<Integer> A_prepared(m.A_rows * m.width);
  Backend::PrepareA(m.A.begin(), A_prepared.begin(), quant_mult, m.A_rows, m.width);
  AlignedVector<Integer> B_prepared(m.width * m.B_cols);
  Backend::PrepareB(m.B.begin(), B_prepared.begin(), quant_mult, m.width, m.B_cols);
  AlignedVector<float> output(m.A_rows * m.B_cols);
  // Burn in
  Backend::Multiply(A_prepared.begin(), B_prepared.begin(), m.A_rows, m.width, m.B_cols, callbacks::UnquantizeAndWrite(unquant_mult, output.begin()));
  auto start = std::chrono::steady_clock::now();
  Backend::Multiply(A_prepared.begin(), B_prepared.begin(), m.A_rows, m.width, m.B_cols, callbacks::UnquantizeAndWrite(unquant_mult, output.begin()));
  return std::chrono::duration<double>(std::chrono::steady_clock::now() - start).count();
}

template <class Backend> void RunAll(RandomMatrices *matrices, RandomMatrices *matrices_end, std::vector<std::vector<double>> &stats) {
  if (Backend::kUses > kCPU) return;
  std::size_t size = matrices_end - matrices;
  if (stats.size() < size)
    stats.resize(size);
  for (std::size_t i = 0; i < size; ++i) {
    stats[i].push_back(Run<Backend>(matrices[i]));
  }
}

struct BackendStats {
  std::vector<std::vector<double>> ssse3_8bit;
  std::vector<std::vector<double>> sse2_16bit;
};

const float kOutlierThreshold = 0.75;
void Summarize(std::vector<double> &stats) {
  // Throw out outliers.
  std::vector<double>::iterator keep = stats.begin() + static_cast<std::size_t>(static_cast<float>(stats.size()) * kOutlierThreshold);
  std::nth_element(stats.begin(), keep, stats.end());
  double avg = 0.0;
  for (std::vector<double>::const_iterator i = stats.begin(); i != keep; ++i) {
    avg += *i;
  }
  avg /= (keep - stats.begin());
  double stddev = 0.0;
  for (std::vector<double>::const_iterator i = stats.begin(); i != keep; ++i) {
    double off = (double)*i - avg;
    stddev += off * off;
  }
  stddev = sqrt(stddev / (keep - stats.begin() - 1));
  std::cout << std::setw(10) << *std::min_element(stats.begin(), stats.end()) << '\t' << std::setw(8) << avg << '\t' << std::setw(8) << stddev;
}

template <class Backend> void Print(std::vector<std::vector<double>> &stats, std::size_t index) {
  if (stats.empty()) return;
  std::cout << std::setw(16) << Backend::kName << '\t';
  Summarize(stats[index]);
  std::cout << '\n';
}

} // namespace intgemm
} // namespace

// Program takes no input
int main(int, char ** argv) {
  using namespace intgemm;
  // The dominant matrix's row count is read from disk so the workload can be
  // resized without recompiling.
  const Index big_rows = (Index)read_int_from_file();
  RandomMatrices matrices[] = {
    {1, 64, 8},
    {8, 2048, 256},
    {big_rows, 256, 256},
  };

  RandomMatrices *matrices_end = (RandomMatrices*)matrices + sizeof(matrices) / sizeof(RandomMatrices);

  bench_start();

  BackendStats stats;
  // One sample is enough: Sightglass has its own sampling loop.
  const int kSamples = 1;
  std::cerr << "SSSE3 8bit, " << kSamples << " samples..." << std::endl;
  for (int samples = 0; samples < kSamples; ++samples) {
    RunAll<SSSE3::Kernels8>(matrices, matrices_end, stats.ssse3_8bit);
  }

  std::cerr << "SSE2 16bit, " << kSamples << " samples..." << std::endl;
  for (int samples = 0; samples < kSamples; ++samples) {
    RunAll<SSE2::Kernels16>(matrices, matrices_end, stats.sse2_16bit);
  }

  if (stats.sse2_16bit.empty()) {
    std::cerr << "No CPU support." << std::endl;
    return 1;
  }

  bench_end();

  return 0;
}
