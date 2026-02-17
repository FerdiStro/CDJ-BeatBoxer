package com.FerdiStro.drum;

import com.FerdiStro.LogUtils;
import com.FerdiStro.drum.beat.Beat;
import com.FerdiStro.drum.command.DrumCommand;
import com.FerdiStro.drum.command.DrumCommandObject;
import com.FerdiStro.drum.modes.AbstractMode;
import com.FerdiStro.memory.SharedMemoryProvider;
import ddf.minim.AudioOutput;
import ddf.minim.Minim;
import ddf.minim.ugens.BitCrush;
import ddf.minim.ugens.Delay;
import ddf.minim.ugens.Summer;
import lombok.Getter;
import lombok.Setter;
import org.apache.logging.log4j.LogManager;
import org.apache.logging.log4j.Logger;

import java.util.HashMap;
import java.util.Map;

public class DrumMachine {


    protected static final Logger log = LogManager.getLogger();
    private final AbstractMode mode;
    private final Minim minim;
    private final AudioOutput audioOutput;

    private final Summer mixer;


    private final Beat[] bigGrid = new Beat[64];

    @Getter
    private final Beat[] smallGrid = new Beat[8];
    /**
     * Map for faster reading
     */
    private final Map<DrumCommand, CallBack> drumSounds = new HashMap<>();


    @Setter
    private Delay echoEffect;

    @Setter
    private BitCrush distortionEffect;

    @Setter
    private Delay reverbEffect;
    private long reverbOffTime = -1;
    private boolean reverbIsPatched = false;

    public DrumMachine(AbstractMode mode) {
        SharedMemoryProvider.getInstance().addMemoryUpdateListeners(mode);

        this.mode = mode;

        this.minim = new Minim(new MinimHelper());
        this.audioOutput = minim.getLineOut();
        if (this.audioOutput == null) {
            throw new RuntimeException("CRITICAL: No Audio-Output found! (Check PI-Audio)");
        }

        this.mixer = new Summer();

        this.echoEffect = new Delay(0.4f, 0.0f, true, true);
        this.distortionEffect = new BitCrush(16.0f, 44100.0f);
        this.reverbEffect = new Delay(0.08f, 0.6f, true, true);

        //Patch effects, expect reverb. This patch on lifetime to save CPU power
        this.mixer.patch(echoEffect).patch(distortionEffect).patch(this.audioOutput);

        drumCommands();
    }


    private Beat getValidBeat(int beatPos) {
        if (mode.isSmallBeat() && beatPos < smallGrid.length) {
            Beat beat = this.smallGrid[beatPos];
            if (beat == null) {
                beat = new Beat(this.minim, this.mixer);
                log.info("Beat_POST: {}", beatPos);
                updateValidBeat(beatPos, beat);
            }
            return beat;
        }
        log.error("Get Big-Beat-Grid: {}", LogUtils.NOT_IMPLEMENTED);
        return null;
    }

    private void updateValidBeat(int beatPos, Beat beat) {
        if (this.mode.isSmallBeat() && beatPos < smallGrid.length) {
            this.smallGrid[beatPos] = beat;
            this.mode.sendMemoryUpdate();
            return;
        }
        log.error("Update Big-Beat-Grid: {}", LogUtils.NOT_IMPLEMENTED);
    }


    private void drumCommands() {
        drumSounds.put(DrumCommand.ADD_SOUND, drumCommandObject -> {
            Beat beat = getValidBeat(drumCommandObject.getBeatPosition());
            assert beat != null;
            beat.addSample(drumCommandObject.getFileName());
            updateValidBeat(drumCommandObject.getBeatPosition(), beat);
        });
        drumSounds.put(DrumCommand.REMOVE_SOUND, drumCommandObject -> {
            Beat beat = getValidBeat(drumCommandObject.getBeatPosition());
            assert beat != null;
            beat.removeSample(drumCommandObject.getFileName());
            updateValidBeat(drumCommandObject.getBeatPosition(), beat);
        });
        drumSounds.put(DrumCommand.EFFECT_ECHO, drumCommandObject -> {
            float strength = map(drumCommandObject.getEffectValue(), 0, 127, 0.0f, 0.8f);
            this.echoEffect.setDelAmp(strength);
        });
        drumSounds.put(DrumCommand.EFFECT_REVERB, drumCommandObject -> {
            int effectValue = drumCommandObject.getEffectValue();
            if (effectValue > 0) {
                if (!reverbIsPatched) {
                    mixer.patch(reverbEffect).patch(audioOutput);
                    reverbIsPatched = true;
                }
                reverbEffect.setDelAmp(map(effectValue, 0, 127, 0.0f, 0.8f));
                reverbOffTime = -1;

                return;
            }

            reverbEffect.setDelAmp(0);

            if (reverbIsPatched && reverbOffTime == -1) {
                reverbOffTime = System.currentTimeMillis();
            }
        });
        drumSounds.put(DrumCommand.EFFECT_DISTORTION, drumCommandObject -> {
            int effectValue = drumCommandObject.getEffectValue();

            if (effectValue == 0) {
                distortionEffect.setBitRes(16.0f);
                distortionEffect.setSampleRate(44100.0f);
                return;
            }

            float bits = map(effectValue, 0, 127, 16.0f, 2.0f);
            float rate = map(effectValue, 0, 127, 44100.0f, 4000.0f);
            distortionEffect.setSampleRate(rate);
            distortionEffect.setBitRes(bits);
        });
        drumSounds.put(DrumCommand.IGNORE, drumCommandObject -> log.info("Command ignored"));
    }

    private float map(float value, float min1, float max1, float min2, float max2) {
        return min2 + (max2 - min2) * ((value - min1) / (max1 - min1));
    }


    public void onCommand(DrumCommandObject drumCommandObject) {
        CallBack callBack = this.drumSounds.get(drumCommandObject.getCommand());
        if (callBack == null) {
            log.error("Drum Command (Command: {}) not implemented", drumCommandObject);
            return;
        }
        callBack.onCallBack(drumCommandObject);
    }


    public void onBeat(int beatPos) {
        Beat beat = getValidBeat(beatPos);

        if (beat != null) {
            try {
                beat.play();

            } catch (Exception e) {
                log.error(e.toString());
                e.printStackTrace();
            }
        }
        this.checkEffectCleanUp();
    }


    /**
     * Check reverb time. Unpatch effect to save CPU-power. validated onBeat(). Can be called from any clock in other thread.
     */
    public void checkEffectCleanUp() {
        if (reverbIsPatched && reverbOffTime != -1 && System.currentTimeMillis() - reverbOffTime > 2000) {
            log.debug("Unpatch Reverb Effect to Save Memory");
            mixer.unpatch(reverbEffect);
            reverbEffect.unpatch(audioOutput);
            reverbIsPatched = false;
            reverbOffTime = -1;
        }

    }

    interface CallBack {
        void onCallBack(DrumCommandObject drumCommandObject);
    }


}
