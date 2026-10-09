package sightglass;

import org.teavm.interop.Address;
import org.teavm.interop.Export;
import org.teavm.interop.Import;

public final class Main {
    // Kept in sync with `default.input`.
    private static final int DEFAULT_FRAMES = 2;

    // Octane's own correctness check: after this many frames the truncated
    // density sum over cells [7000, 7100) is 77.
    private static final int CHECK_FRAME = 15;
    private static final int EXPECTED_CHECK = 77;

    // Linear memory only holds TeaVM's static data (at address 0) and these
    // WASI argument buffers; Java objects are all GC structs/arrays. `shim.wat`
    // places TeaVM's malloc heap above the first 64 KiB page.
    private static final int SCRATCH = 0x8000;
    private static final int SCRATCH_LEN = 256;
    private static final int OUT_BUF = SCRATCH + SCRATCH_LEN;
    private static final int OUT_BUF_LEN = 0x1000;

    private static final int PREOPEN_FD = 3;
    private static final int STDOUT_FD = 1;
    private static final long RIGHTS_FD_READ = 2L;

    @Import(module = "bench", name = "start")
    private static native void benchStart();

    @Import(module = "bench", name = "end")
    private static native void benchEnd();

    @Import(module = "wasi_snapshot_preview1", name = "path_open")
    private static native int pathOpen(int fd, int dirflags, int path, int pathLen, int oflags,
            long rightsBase, long rightsInheriting, int fdflags, int resultFd);

    @Import(module = "wasi_snapshot_preview1", name = "fd_read")
    private static native int fdRead(int fd, int iovs, int iovsLen, int nread);

    @Import(module = "wasi_snapshot_preview1", name = "fd_write")
    private static native int fdWrite(int fd, int iovs, int iovsLen, int nwritten);

    @Import(module = "wasi_snapshot_preview1", name = "fd_close")
    private static native int fdClose(int fd);

    private static int framesTillAddingPoints = 0;
    private static int framesBetweenAddingPoints = 5;

    private static void addPoints(FluidField.Field field) {
        int n = 64;
        for (int i = 1; i <= n; i++) {
            field.setVelocity(i, i, n, n);
            field.setDensity(i, i, 5);
            field.setVelocity(i, n - i, -n, -n);
            field.setDensity(i, n - i, 20);
            field.setVelocity(128 - i, n + i, -n, -n);
            field.setDensity(128 - i, n + i, 30);
        }
    }

    private static void prepareFrame(FluidField.Field field) {
        if (framesTillAddingPoints == 0) {
            addPoints(field);
            framesTillAddingPoints = framesBetweenAddingPoints;
            framesBetweenAddingPoints++;
        } else {
            framesTillAddingPoints--;
        }
    }

    private static int octaneCheck(double[] dens) {
        int result = 0;
        for (int i = 7000; i < 7100; i++) {
            result += (int) (dens[i] * 10);
        }
        return result;
    }

    @Export(name = "_start")
    public static void start() {
        int frames = readInt("./default.input", DEFAULT_FRAMES);

        FluidField solver = new FluidField();
        solver.setResolution(128, 128);
        solver.setIterations(20);
        solver.setUICallback(Main::prepareFrame);
        solver.reset();

        benchStart();
        int check = -1;
        for (int frame = 1; frame <= frames; frame++) {
            solver.update();
            if (frame == CHECK_FRAME) {
                check = octaneCheck(solver.getDens());
            }
        }
        benchEnd();

        double[] dens = solver.getDens();
        long hash = 0;
        for (int i = 0; i < dens.length; i++) {
            hash = hash * 31 + Double.doubleToRawLongBits(dens[i]);
        }

        Output out = new Output();
        out.text("frames: ").decimal(frames).text("\ndensity hash: ").hex(hash).text("\n");
        if (check >= 0) {
            out.text(check == EXPECTED_CHECK ? "octane check: ok\n" : "octane check: FAILED\n");
        }
        out.flush();
    }

    private static Address scratch(int offset) {
        return Address.fromInt(SCRATCH + offset);
    }

    // Returns the first decimal integer in the file at `path`, or `fallback`
    // if the file cannot be read or holds no integer.
    private static int readInt(String path, int fallback) {
        char[] cs = path.toCharArray();
        int pathLen = cs.length;
        int pathAt = 64;
        for (int i = 0; i < pathLen; i++) {
            scratch(pathAt + i).putByte((byte) cs[i]);
        }
        if (pathOpen(PREOPEN_FD, 0, SCRATCH + pathAt, pathLen, 0, RIGHTS_FD_READ, 0L, 0, SCRATCH) != 0) {
            return fallback;
        }
        int fd = scratch(0).getInt();

        int bufAt = 128;
        int bufLen = SCRATCH_LEN - bufAt;
        scratch(8).putInt(SCRATCH + bufAt);
        scratch(12).putInt(bufLen);
        int rc = fdRead(fd, SCRATCH + 8, 1, SCRATCH + 16);
        int n = scratch(16).getInt();
        fdClose(fd);
        if (rc != 0) {
            return fallback;
        }

        int value = 0;
        boolean sawDigit = false;
        for (int i = 0; i < n; i++) {
            int c = scratch(bufAt + i).getByte();
            if (c >= '0' && c <= '9') {
                value = value * 10 + (c - '0');
                sawDigit = true;
            } else if (sawDigit) {
                break;
            }
        }
        return sawDigit ? value : fallback;
    }

    // Buffers ASCII output in linear memory and writes it with one `fd_write`.
    private static final class Output {
        private int len;

        Output text(String s) {
            char[] cs = s.toCharArray();
            for (int i = 0; i < cs.length; i++) {
                put(cs[i]);
            }
            return this;
        }

        Output decimal(long v) {
            long div = 1;
            while (v / div >= 10) {
                div *= 10;
            }
            for (; div > 0; div /= 10) {
                put((char) ('0' + (int) ((v / div) % 10)));
            }
            return this;
        }

        Output hex(long v) {
            for (int shift = 60; shift >= 0; shift -= 4) {
                int d = (int) (v >>> shift) & 0xf;
                put((char) (d < 10 ? '0' + d : 'a' + d - 10));
            }
            return this;
        }

        private void put(char c) {
            if (len < OUT_BUF_LEN) {
                Address.fromInt(OUT_BUF + len).putByte((byte) c);
                len++;
            }
        }

        void flush() {
            int ptr = OUT_BUF;
            while (len > 0) {
                scratch(0).putInt(ptr);
                scratch(4).putInt(len);
                if (fdWrite(STDOUT_FD, SCRATCH, 1, SCRATCH + 8) != 0) {
                    return;
                }
                int n = scratch(8).getInt();
                ptr += n;
                len -= n;
            }
        }
    }
}
