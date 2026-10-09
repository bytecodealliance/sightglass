using System;
using System.IO;
using RayTracer;
using DotnetRaytracerWorld.wit.Imports;

static class Program
{
    // Fallbacks for when ./default.input is absent; keep in sync with it.
    const int DefaultBatchSize = 8;
    const int DefaultSceneWidth = 32;
    const int DefaultSceneHeight = 32;

    static int Main()
    {
        int[] knobs = { DefaultBatchSize, DefaultSceneWidth, DefaultSceneHeight };
        try
        {
            // Raw handle and byte I/O keep FileStream, StreamReader, Console's
            // TextWriter, and the text encodings out of the binary, which
            // matters for compile time.
            using var file = File.OpenHandle("./default.input");
            var input = new byte[256];
            int len = RandomAccess.Read(file, input, 0);
            int k = 0;
            for (int i = 0; i < len && k < knobs.Length; i++)
            {
                if (input[i] < '0' || input[i] > '9')
                    continue;
                int v = 0;
                for (; i < len && input[i] >= '0' && input[i] <= '9'; i++)
                    v = v * 10 + (input[i] - '0');
                knobs[k++] = v;
            }
        }
        catch (IOException)
        {
        }
        int batchSize = knobs[0], sceneWidth = knobs[1], sceneHeight = knobs[2];

        IBenchImports.Start();

        // Same shape as JetStream's `RunIteration`: the exception micro-tasks,
        // then one frame of the ray tracer.
        var exceptions = new Sample.ExceptionsTask();
        for (int j = 0; j < exceptions.Measurements.Length; j++)
        {
            exceptions.RunBatch(j, batchSize);
        }

        var scene = Scene.TwoPlanes;
        scene.Camera.ReflectionDepth = 5;
        scene.Camera.FieldOfView = 120;
        var rgba = new byte[sceneWidth * sceneHeight * 4];
        scene.Camera.RenderScene(scene, rgba, sceneWidth, sceneHeight, 1);

        IBenchImports.End();

        uint hash = 2166136261;
        foreach (var b in rgba)
        {
            hash = (hash ^ b) * 16777619;
        }

        var o = new Output();
        o.Str("caught "); o.Dec((uint)Sample.ExceptionsTask.Caught); o.Str(" exceptions\n");
        o.Str("rendered "); o.Dec((uint)sceneWidth); o.Str("x"); o.Dec((uint)sceneHeight);
        o.Str(", fnv1a "); o.Hex(hash); o.Str("\n");
        using var stdout = Console.OpenStandardOutput();
        stdout.Write(o.Buf, 0, o.Len);
        return 0;
    }

    // ASCII-only formatting, avoiding the BCL's number formatting code.
    sealed class Output
    {
        public byte[] Buf = new byte[128];
        public int Len;

        public void Str(string s)
        {
            foreach (char c in s)
                Buf[Len++] = (byte)c;
        }

        public void Dec(uint v)
        {
            int start = Len;
            do { Buf[Len++] = (byte)('0' + v % 10); v /= 10; } while (v != 0);
            Array.Reverse(Buf, start, Len - start);
        }

        public void Hex(uint v)
        {
            for (int shift = 28; shift >= 0; shift -= 4)
                Buf[Len++] = (byte)"0123456789abcdef"[(int)(v >> shift) & 0xf];
        }
    }
}
