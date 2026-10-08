// Sightglass driver for the DeltaBlue port. dart2wasm's standalone mode has no
// file I/O library, so WASI preview1 and the `bench` hooks are bound through
// experimental `dart:ffi` natives, which become plain core-module imports.
// `print` goes to the `dart.print` embedder import; see `embedder.wat`.
@DefaultAsset('wasi_snapshot_preview1')
library;

import 'dart:ffi';

import 'deltablue.dart';

/// Kept in sync with `default.input`.
const defaultIterations = 86;

@Native<Void Function()>(symbol: 'start', assetId: 'bench')
external void benchStart();

@Native<Void Function()>(symbol: 'end', assetId: 'bench')
external void benchEnd();

@Native<
  Int32 Function(
    Int32,
    Int32,
    Pointer<Uint8>,
    Int32,
    Int32,
    Int64,
    Int64,
    Int32,
    Pointer<Int32>,
  )
>(symbol: 'path_open')
external int pathOpen(
  int fd,
  int dirflags,
  Pointer<Uint8> path,
  int pathLen,
  int oflags,
  int rightsBase,
  int rightsInheriting,
  int fdflags,
  Pointer<Int32> openedFd,
);

@Native<Int32 Function(Int32, Pointer<Uint32>, Int32, Pointer<Uint32>)>(
  symbol: 'fd_read',
)
external int fdRead(
  int fd,
  Pointer<Uint32> iovs,
  int iovsLen,
  Pointer<Uint32> nread,
);

@Native<Int32 Function(Int32)>(symbol: 'fd_close')
external int fdClose(int fd);

// Scratch layout in the imported linear memory, which the build gives one
// page. There is no allocator, so addresses are fixed.
const _iovAddr = 0;
const _countAddr = 8;
const _fdAddr = 12;
const _bufAddr = 16;
const _bufLen = 1024;

/// The harness preopens the benchmark's directory as the first preopen.
const _preopenFd = 3;
const _rightsFdRead = 2;

final _buf = Pointer<Uint8>.fromAddress(_bufAddr);

int _writeAscii(int at, String s) {
  for (var i = 0; i < s.length; i++) {
    _buf[at++] = s.codeUnitAt(i);
  }
  return at;
}

/// Returns the leading decimal integer in `path`, or `fallback` if the file
/// cannot be read or has none.
int readIterations(String path, int fallback) {
  var len = _writeAscii(0, path);
  var fd = Pointer<Int32>.fromAddress(_fdAddr);
  if (pathOpen(_preopenFd, 0, _buf, len, 0, _rightsFdRead, 0, 0, fd) != 0) {
    return fallback;
  }
  var iov = Pointer<Uint32>.fromAddress(_iovAddr);
  iov[0] = _bufAddr;
  iov[1] = _bufLen;
  var nread = Pointer<Uint32>.fromAddress(_countAddr);
  var rc = fdRead(fd.value, iov, 1, nread);
  fdClose(fd.value);
  if (rc != 0) return fallback;
  var value = 0;
  var sawDigit = false;
  for (var i = 0; i < nread.value; i++) {
    var c = _buf[i];
    if (c >= 0x30 && c <= 0x39) {
      value = value * 10 + (c - 0x30);
      sawDigit = true;
    } else if (sawDigit) {
      break;
    }
  }
  return sawDigit ? value : fallback;
}

void main() {
  var iterations = readIterations('default.input', defaultIterations);

  benchStart();
  var marks = 0;
  for (var i = 0; i < iterations; i++) {
    marks += deltaBlue();
  }
  benchEnd();

  print('[dart-deltablue] iterations: $iterations');
  print('[dart-deltablue] marks: $marks');
  print(alerts == 0 ? '[dart-deltablue] verified' : '[dart-deltablue] FAILED');
}
