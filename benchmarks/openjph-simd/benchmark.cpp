#include <iostream>
#include <cstdlib>

#include "ojph_file.h"
#include "ojph_codestream.h"
#include "ojph_params.h"
#include "ojph_mem.h"

#include "sightglass.h"

#include "sample_jph.c"

int main(int argc, char *argv[]) {
  ojph::mem_infile j2c_file;
  j2c_file.open(sample_jph, sample_jph_len);
  ojph::codestream codestream;
  codestream.read_headers(&j2c_file);
  ojph::param_siz siz = codestream.access_siz();
  std::cout << "Size: " << siz.get_recon_width(0) << 
      "x" << siz.get_recon_height(0) << std::endl;

  bench_start();
  codestream.create();
  ojph::ui32 height = siz.get_recon_height(0);
  for (ojph::ui32 i = 0; i < height; ++i) {
    if (i == 100)
        std::cout << "Line 100:" << std::endl;
    for (ojph::ui32 c = 0; c < siz.get_num_components(); ++c) {
      ojph::ui32 comp_num;
      ojph::line_buf *line = codestream.pull(comp_num);
      assert(comp_num == c);
      if (i == 100) {
        std::cout << " [" << c << "]: ";
        for (int j = 0; j < line->size; j++)
          std::cout << " " << line->i32[j];
        std::cout << std::endl;
      }
    }
  }
  codestream.close();
  bench_end();

  return 0;
}
