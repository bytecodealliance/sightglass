// Licensed to the .NET Foundation under one or more agreements.
// The .NET Foundation licenses this file to you under the MIT license.

using System;

public abstract class BenchTask
{
    public abstract string Name { get; }

    public void RunBatch(int measurementIdx, int batchSize)
    {
        Measurements[measurementIdx].RunBatch(batchSize);
    }

    public abstract Measurement[] Measurements { get; }

    public abstract class Measurement
    {
        protected int currentStep = 0;
        public abstract string Name { get; }

        public virtual int InitialSamples => 2;

        public virtual void RunStep() { }

        public void RunBatch(int batchSize)
        {
            int initialSamples = InitialSamples;
            try
            {
                for (currentStep = 0; currentStep < initialSamples * batchSize; currentStep++)
                {
                    RunStep();
                }
            }
            catch (Exception)
            {
            }
        }
    }
}
