package com.FerdiStro.drum.beat;

import ddf.minim.Minim;
import ddf.minim.ugens.Sampler;
import ddf.minim.ugens.Summer;
import lombok.Getter;
import org.apache.logging.log4j.LogManager;
import org.apache.logging.log4j.Logger;

public class Beat {
    protected static final Logger log = LogManager.getLogger();
    private static final int MAX_SAMPLER_SIZE = 5;
    //    private static final String SAMPLE_KICK = "/home/ferdinoond/CDJ-BeatBoxer/BeatBoxer-Sounds/KICK_20.wav";
    private final Sampler[] samplers;
    @Getter
    private final String[] samplersNames;
    private final Minim minim;
    private final Summer mixer;

    public Beat(Minim minim, Summer mixer) {
        this.samplers = new Sampler[MAX_SAMPLER_SIZE];
        this.samplersNames = new String[MAX_SAMPLER_SIZE];
        this.minim = minim;
        this.mixer = mixer;
    }

    public void addSample(String filePath) {
        Sampler sampler = new Sampler(filePath, 4, minim);
        sampler.patch(mixer);

        boolean added = false;
        for (int i = 0; i < samplers.length; i++) {
            if (samplers[i] == null) {
                samplers[i] = sampler;
                samplersNames[i] = filePath;
                added = true;
                break;
            }
        }
        if (!added) {
            log.error("Beat is full! Cannot add more samples.");
        }
    }

    public void play() {
        for (Sampler sampler : samplers) {
            if (sampler != null) {
                sampler.trigger();
            }
        }

    }
}
