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
import ddf.minim.ugens.Sampler;
import ddf.minim.ugens.Summer;
import lombok.Getter;
import lombok.Setter;
import org.apache.logging.log4j.LogManager;
import org.apache.logging.log4j.Logger;

import java.util.Arrays;
import java.util.EnumMap;
import java.util.Map;

public class DrumMachine {


    protected static final Logger log = LogManager.getLogger();
    //defines the time in cache (count via beats)
    private static final int ON_SHOOT_CACHE_TIME = 10;


    private final AbstractMode mode;

    private final Minim minim;
    private final AudioOutput audioOutput;
    private final Summer mixer;


    /**
     * OnShoot play-grid 8 bar long. When [][] is true play sound. Get index form onShootCacheRef;
     */
    private final boolean[][] onShootsPlayOnBeat = new boolean[8][3];
    /**
     * OnShoot cache refs
     */
    private final int[][] onShootCacheRef = new int[8][3];
    private final int[] onShootBeatRef = new int[8];
    /**
     * OnShoots cached time stamps. Used to check cache lifecycle
     */
    private final int[] onShootCacheTimeStamps = new int[8];
    /**
     * OnShoots cached Sampler. This is the cached object already patched to the mixer
     */
    private final Sampler[] cachedOnShoots = new Sampler[8];
    /**
     * OnShoots cached file paths. This array stores all file paths for faster search
     */
    private final String[] onShootsOnBeatPaths = new String[8];


    private final Beat[] bigGrid = new Beat[64];

    @Getter
    private final Beat[] smallGrid = new Beat[8];

    /**
     * Map for faster reading
     */
    private final Map<DrumCommand, CallBack> drumCommandCallBackMap = new EnumMap<>(DrumCommand.class);

    @Getter
    @Setter
    private boolean onShootMode;

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


        //init Cache arrays
        for (int[] ints : onShootCacheRef) {
            Arrays.fill(ints, -1);
        }
        Arrays.fill(onShootBeatRef, -1);
        Arrays.fill(onShootCacheTimeStamps, -1);

        //load drum commands
        drumCommands();
    }


    private void drumCommands() {
        drumCommandCallBackMap.put(DrumCommand.ADD_SOUND, drumCommandObject -> {
            if (!isOnShootMode()) {
                addSound(drumCommandObject);
                return;
            }
            addOnShoot(drumCommandObject);
        });
        drumCommandCallBackMap.put(DrumCommand.REMOVE_SOUND, drumCommandObject -> {
            Beat beat = getValidBeat(drumCommandObject.getBeatPosition());
            assert beat != null;
            beat.removeSample(drumCommandObject.getFileName());
            updateValidBeat(drumCommandObject.getBeatPosition(), beat);
        });
        drumCommandCallBackMap.put(DrumCommand.EFFECT_ECHO, drumCommandObject -> {
            float strength = map(drumCommandObject.getEffectValue(), 0, 127, 0.0f, 0.8f);
            this.echoEffect.setDelAmp(strength);
        });
        drumCommandCallBackMap.put(DrumCommand.EFFECT_REVERB, drumCommandObject -> {
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
        drumCommandCallBackMap.put(DrumCommand.EFFECT_DISTORTION, drumCommandObject -> {
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
        drumCommandCallBackMap.put(DrumCommand.ON_SHOOT_MODE, drumCommandObject -> setOnShootMode(!isOnShootMode()));
        drumCommandCallBackMap.put(DrumCommand.IGNORE, drumCommandObject -> log.info("Command ignored"));
    }

    private float map(float value, float min1, float max1, float min2, float max2) {
        return min2 + (max2 - min2) * ((value - min1) / (max1 - min1));
    }


    public void onCommand(DrumCommandObject drumCommandObject) {
        CallBack callBack = this.drumCommandCallBackMap.get(drumCommandObject.getCommand());
        if (callBack == null) {
            log.error("Drum Command (Command: {}) not implemented", drumCommandObject);
            return;
        }
        callBack.onCallBack(drumCommandObject);
    }

//-----------------OnBeat handling (extern clock)-----------------

    public void onBeat(int beatPos) {
        //Beat on Grid
        Beat beat = getValidBeat(beatPos);
        if (beat != null) {
            try {
                beat.play();

            } catch (Exception e) {
                log.error(e.toString());
                e.printStackTrace();
            }
        }
        //OnShoots on grid
        boolean[] onShoots = onShootsPlayOnBeat[beatPos];
        for (int i = 0; i < onShoots.length; i++) {
            if (onShoots[i]) {
                this.playOnShoot(beatPos, i);
            }
        }
        this.checkCacheCleanUp();
    }


    /**
     * Checks cached sounds/effects and clean up to save CPU/Memory.
     * Called on beat. Can also be called from any other loop/Thread
     */
    public void checkCacheCleanUp() {
        //effect cache
        this.checkEffectCleanUp();
        //onShoot cache
        this.checkOnShootClenUp();
    }


//-----------------Effect handling -----------------

    /**
     * Check reverb time. Unpatch effect to save CPU-power.
     */
    private void checkEffectCleanUp() {
        if (reverbIsPatched && reverbOffTime != -1 && System.currentTimeMillis() - reverbOffTime > 2000) {
            log.debug("Unpatch Reverb Effect to Save Memory");
            mixer.unpatch(reverbEffect);
            reverbEffect.unpatch(audioOutput);
            reverbIsPatched = false;
            reverbOffTime = -1;
        }

    }
//-----------------Beat handling -----------------

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


    private void addSound(DrumCommandObject drumCommandObject) {
        Beat beat = getValidBeat(drumCommandObject.getBeatPosition());
        assert beat != null;
        beat.addSample(drumCommandObject.getFileName());
        updateValidBeat(drumCommandObject.getBeatPosition(), beat);
    }

//-----------------OnShoot handling -----------------

    private void playOnShoot(int beatPos, int cacheIndex) {
        int onShottRefIndex = this.onShootCacheRef[beatPos][cacheIndex];
        Sampler onShootSample = cachedOnShoots[onShottRefIndex];
        onShootSample.trigger();
        this.onShootsPlayOnBeat[beatPos][cacheIndex] = false;
    }


    private void addOnShoot(DrumCommandObject drumCommandObject) {
        int beatPosition = drumCommandObject.getBeatPosition();
        String filePath = drumCommandObject.getFileName();

        int onShootIndexFromCache = this.getOnShootFromCache(beatPosition, filePath);

        if (onShootIndexFromCache != -1) {
            this.updateOnShootFromCache(onShootIndexFromCache, beatPosition);
            return;
        }

        this.putOnShootToCache(beatPosition, filePath);

        int newCacheIndex = this.getOnShootFromCache(beatPosition, filePath);

        if (newCacheIndex == -1) {
            log.error("OnShoot caching not work!");
        }
    }


    private void updateOnShootFromCache(int cacheIndex, int beatRef) {
        onShootCacheTimeStamps[cacheIndex] = ON_SHOOT_CACHE_TIME;
        onShootsPlayOnBeat[beatRef][cacheIndex] = true;
    }

    private void deleteOnShootFromCache(int cacheIndex, int beatRef) {
        onShootCacheTimeStamps[cacheIndex] = -1;
        onShootBeatRef[cacheIndex] = -1;
        cachedOnShoots[cacheIndex] = null;
        onShootsOnBeatPaths[cacheIndex] = null;
        onShootsPlayOnBeat[beatRef][cacheIndex] = false;
        onShootCacheRef[beatRef][cacheIndex] = -1;
    }

    private int getOnShootFromCache(int beatRef, String filePath) {
        int[] cacheIndexes = onShootCacheRef[beatRef];

        for (int cacheIndex = 0; cacheIndex < cacheIndexes.length; cacheIndex++) {
            int onShootCachePath = cacheIndexes[cacheIndex];
            if (onShootCachePath != -1 && onShootsOnBeatPaths[cacheIndex].equals(filePath)) {

                return onShootCacheRef[beatRef][cacheIndex];
            }
        }
        return -1;
    }

    private void putOnShootToCache(int beatRef, String filePath) {
        boolean addedToCache = false;
        for (int cachedIndex = 0; cachedIndex < cachedOnShoots.length; cachedIndex++) {
            Sampler cachedOnShoot = cachedOnShoots[cachedIndex];
            if (cachedOnShoot == null) {
                Sampler sampler = new Sampler(filePath, 4, minim);
                sampler.patch(mixer);

                cachedOnShoots[cachedIndex] = sampler;
                onShootCacheTimeStamps[cachedIndex] = ON_SHOOT_CACHE_TIME;
                onShootsPlayOnBeat[beatRef][cachedIndex] = true;
                onShootBeatRef[cachedIndex] = beatRef;
                onShootsOnBeatPaths[cachedIndex] = filePath;

                int[] cacheRef = onShootCacheRef[beatRef];

                for (int soundCacheIndex = 0; soundCacheIndex < cacheRef.length; soundCacheIndex++) {
                    int ref = cacheRef[soundCacheIndex];
                    if (ref == -1) {
                        cacheRef[soundCacheIndex] = cachedIndex;
                        addedToCache = true;
                        break;
                    }
                }
                break;
            }
        }

        if (addedToCache) {
            return;
        }

        log.warn("OnShoot Sample Cache is full. Remove sample with lowest lifetime (still not finished the caching cycle) ");

        int lowestTimeStampIndex = -1;
        int lowestTimeTamp = -1;

        for (int i = 0; i < onShootCacheTimeStamps.length; i++) {
            int onShootCacheTimeStamp = onShootCacheTimeStamps[i];
            if (lowestTimeTamp < onShootCacheTimeStamp) {
                lowestTimeStampIndex = i;
                lowestTimeTamp = onShootCacheTimeStamp;
            }
        }

        deleteOnShootFromCache(lowestTimeStampIndex, beatRef);
        putOnShootToCache(beatRef, filePath);
    }

    private void checkOnShootClenUp() {
        for (int cacheIndex = 0; cacheIndex < onShootCacheTimeStamps.length; cacheIndex++) {
            int onShootCacheTimeStamp = onShootCacheTimeStamps[cacheIndex];
            if (onShootCacheTimeStamp != -1) {
                onShootCacheTimeStamp -= 1;

                if (onShootCacheTimeStamp <= -1) {
                    int beatRef = this.onShootBeatRef[cacheIndex];
                    this.deleteOnShootFromCache(cacheIndex, beatRef);
                }
                onShootCacheTimeStamps[cacheIndex] = onShootCacheTimeStamp;
            }
        }
    }


    /**
     * CallBack interface for DrumCommands
     */
    interface CallBack {
        void onCallBack(DrumCommandObject drumCommandObject);
    }
}
