package com.FerdiStro.memory.objects;

import com.FerdiStro.drum.beat.Beat;
import com.FerdiStro.memory.exceptions.ByteSizeValidationException;
import lombok.AllArgsConstructor;
import lombok.Getter;
import org.apache.logging.log4j.LogManager;
import org.apache.logging.log4j.Logger;

import java.nio.MappedByteBuffer;
import java.nio.charset.StandardCharsets;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;


public class TransferObject {

    protected static final Logger log = LogManager.getLogger();

    private static final int POSITION_BPM = 8;
    private static final int POSITION_SMALL_COUNTER = 16;
    private static final int POSITION_MASTER = 17;
    private static final int POSITION_TOTAL_COUNTER = 24;
    private static final int POSITION_SOUNDS = 32;
    private static final int BLOCK_SOUNDS_SIZE = 264;


    @Getter
    private final double bpm;
    @Getter
    private final byte smallCounter;
    private final long totalCounter;
    @Getter
    private final boolean master;
    @Getter
    private final TransferBeatData[] transferBeatData;

    public TransferObject(double bpm, byte smallCounter, long totalCounter, boolean master, Beat[] beats) {
        byte maxSize = 0b100;
        if (smallCounter > maxSize) {
            throw new ByteSizeValidationException("Small-Counter", maxSize, smallCounter);
        }
        this.bpm = bpm;
        this.smallCounter = smallCounter;
        this.totalCounter = totalCounter;
        this.master = master;
        this.transferBeatData = fromBeats(beats);
    }

    public void writeMappedByteBuffer(MappedByteBuffer buffer, long sequence) {
        if (buffer.isReadOnly()) {
            log.error("Buffer is read only");
            return;
        }
        buffer.position(0);
        buffer.putDouble(POSITION_BPM, bpm);
        buffer.put(POSITION_SMALL_COUNTER, smallCounter);
        buffer.put(POSITION_MASTER, this.master ? (byte) 1 : (byte) 0);
        buffer.putLong(POSITION_TOTAL_COUNTER, totalCounter);


        for (int i = 0; i < 10; i++) {
            int currentPos = POSITION_SOUNDS + (i * BLOCK_SOUNDS_SIZE);
            buffer.position(currentPos);

            TransferBeatData data = transferBeatData[i];

            String pathToWrite = (data != null && data.path != null) ? data.path : "";
            byte[] pathBytes = pathToWrite.getBytes(StandardCharsets.UTF_8);


            int lenToWrite = Math.min(pathBytes.length, 256);
            buffer.put(pathBytes, 0, lenToWrite);
            for (int k = lenToWrite; k < 256; k++) {
                buffer.put((byte) 0);
            }

            byte bitmask = (data != null) ? data.activeBeatMask : 0;
            buffer.put(currentPos + 256, bitmask);
        }


        buffer.putLong(0, sequence);
    }

    private TransferBeatData[] fromBeats(Beat[] beats) {
        if (beats == null || beats.length != 8) {
            throw new ByteSizeValidationException("Beats-Array", (byte) 8, (byte) beats.length);
        }

        Map<String, Integer> soundMap = new HashMap<>();

        for (int beatIndex = 0; beatIndex < beats.length; beatIndex++) {
            Beat beat = beats[beatIndex];
            if (beat == null) continue;

            String[] samplerNames = beat.getSamplersNames();
            if (samplerNames == null) continue;

            for (String soundPath : samplerNames) {
                if (soundPath != null) {
                    int currentMask = soundMap.getOrDefault(soundPath, 0);


                    currentMask |= (1 << beatIndex);

                    soundMap.put(soundPath, currentMask);
                }
            }
        }

        TransferBeatData[] result = new TransferBeatData[10];
        List<String> keys = new ArrayList<>(soundMap.keySet());


        int limit = Math.min(keys.size(), 10);

        for (int i = 0; i < limit; i++) {
            String path = keys.get(i);
            int maskInt = soundMap.get(path);
            result[i] = new TransferBeatData(path, (byte) maskInt);
        }

        return result;
    }


    @AllArgsConstructor
    public static class TransferBeatData {
        public String path;
        public byte activeBeatMask;


    }


}


